use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::ops::Bound;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use toml_span::Deserialize;
use toml_span::de_helpers::TableHelper;
use toml_span::value::{Value, ValueInner};

const DIM: &str = "\x1b[0;37m";
const GREEN: &str = "\x1b[1;32m";
const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

fn errorln(msg: &str) {
    eprintln!("{RED}[ERROR] {msg}{RESET}");
}

static PROJECT_DIR: LazyLock<&Path> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("Could not locate project root")
});
static SOLUTIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PROJECT_DIR.join("solutions"));

fn rs_files_in_dir(read_dir: fs::ReadDir) -> Vec<PathBuf> {
    let mut rs_files: Vec<_> = read_dir
        .map(|maybe_entry| maybe_entry.expect("Failed to access an entry."))
        .filter_map(|entry| {
            if entry.metadata().is_ok_and(|m| !m.is_file()) {
                return None;
            }
            if let path = entry.path()
                && matches!(path.extension(), Some(ext) if ext == "rs")
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    rs_files.sort(); // Ensure the ascending order for file names across platforms
    rs_files
}

#[derive(Debug)]
struct BinTarget {
    name: ExerciseKey,
    path: PathBuf,
}

impl<'de> Deserialize<'de> for BinTarget {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, toml_span::DeserError> {
        use toml_span::ErrorKind::Custom;
        use toml_span::{Error, Spanned};

        let mut th = TableHelper::new(value)?;

        let name: ExerciseKey = th.required("name")?;

        let rel_path = th.required_s::<String>("path")?;
        let abs_path = Spanned::with_span(PROJECT_DIR.join(rel_path.value), rel_path.span);
        let file_name = abs_path.value.file_name().unwrap();

        if two_digit_prefix(file_name).is_none_or(|prefix| prefix.as_bytes() != name.digits) {
            let kind = Custom("The prefix of the path and the name does not match.".into());
            return Err(Error::from((kind, abs_path.span)).into());
        }
        let path = abs_path.value;

        th.finalize(None)?;
        Ok(BinTarget { name, path })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ExerciseKey {
    digits: [u8; 2],
}

impl ExerciseKey {
    fn start() -> Self {
        let digits = [b'0', b'0'];
        ExerciseKey { digits }
    }
}

impl std::str::FromStr for ExerciseKey {
    type Err = toml_span::DeserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ExerciseKey::deserialize(&mut Value::new(ValueInner::String(s.into())))
    }
}

impl std::fmt::Display for ExerciseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.digits[0] as char, self.digits[1] as char)
    }
}

impl<'de> Deserialize<'de> for ExerciseKey {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, toml_span::DeserError> {
        use toml_span::ErrorKind::Custom;
        use toml_span::{DeserError, Error};
        let maybe_digits = value.take_string(None).map_err(DeserError::from)?;
        let digits = if let Some((chunk, rest)) = maybe_digits.as_bytes().split_first_chunk::<2>()
            && rest.is_empty()
            && chunk[0].is_ascii_digit()
            && chunk[1].is_ascii_digit()
        {
            *chunk
        } else {
            let kind = Custom("The name of a binary target must be two digits.".into());
            return Err(DeserError::from(Error::from((kind, value.span))));
        };
        Ok(ExerciseKey { digits })
    }
}

#[derive(Debug)]
struct Progress {
    version: String,
    current: ExerciseKey,
    exercises: BTreeMap<ExerciseKey, ExerciseStatus>,
}

#[derive(Debug, Clone, Copy)]
enum ExerciseStatus {
    Untouched,
    Partial,
    Solved,
}

impl std::str::FromStr for ExerciseStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "untouched" => Ok(ExerciseStatus::Untouched),
            "partial" => Ok(ExerciseStatus::Partial),
            "solved" => Ok(ExerciseStatus::Solved),
            status => Err(status.to_string()),
        }
    }
}

impl std::fmt::Display for ExerciseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ExerciseStatus::Untouched => "untouched",
            ExerciseStatus::Partial => "partial",
            ExerciseStatus::Solved => "solved",
        };
        write!(f, "{}", repr)
    }
}

impl Progress {
    const FILE_NAME: &str = "progress.toml";

    fn new(version: String, bin_paths: &BTreeMap<ExerciseKey, PathBuf>) -> Self {
        let mut exercises = BTreeMap::new();
        for key in bin_paths.keys() {
            exercises.insert(*key, ExerciseStatus::Untouched);
        }
        Progress {
            version,
            current: ExerciseKey::start(),
            exercises,
        }
    }

    fn load() -> Option<Self> {
        let progress_toml = PROJECT_DIR.join(Self::FILE_NAME);
        // NOTE: Path::exists method is not ideal because of TOCTOU concerns
        if !progress_toml.exists() {
            return None;
        }

        let content = fs::read_to_string(progress_toml).expect("Failed to read progress.toml");
        let toml_value = &mut toml_span::parse(&content).expect("Failed to parse progress.toml");
        let mut th = TableHelper::new(toml_value).unwrap();

        let version: String = th.required("version").unwrap();
        let current: ExerciseKey = th.required("current").unwrap();
        struct Exercises(BTreeMap<ExerciseKey, ExerciseStatus>);
        impl<'de> Deserialize<'de> for Exercises {
            fn deserialize(value: &mut Value<'de>) -> Result<Self, toml_span::DeserError> {
                use toml_span::de_helpers::expected;

                let mut exercises = BTreeMap::new();

                let ValueInner::Table(tbl) = value.take() else {
                    return Err(expected("Must be a table.", value.take(), value.span).into());
                };
                for (key, mut value) in tbl {
                    let key: ExerciseKey = str::parse(&key.name)?;
                    let Some(value) = value.as_str() else {
                        return Err(
                            expected("Only string is allowed.", value.take(), value.span).into(),
                        );
                    };
                    let progress: ExerciseStatus = value.parse().expect("Unknown status");
                    exercises.insert(key, progress);
                }

                Ok(Exercises(exercises))
            }
        }
        let Exercises(exercises) = th.required("exercises").unwrap();
        th.finalize(None).unwrap();

        Some(Progress {
            version,
            current,
            exercises,
        })
    }

    fn reset(&mut self) {
        self.current = ExerciseKey::start();
        for status in self.exercises.values_mut() {
            *status = ExerciseStatus::Untouched;
        }
    }

    fn update(&mut self, new_version: &str, bin_paths: &BTreeMap<ExerciseKey, PathBuf>) {
        self.version = new_version.to_string();
        for key in bin_paths.keys() {
            self.exercises.entry(*key).or_insert_with(|| {
                println!("Added new exercise, {key}.");
                ExerciseStatus::Untouched
            });
        }

        for key in self.exercises.keys() {
            if !bin_paths.contains_key(key) {
                eprintln!(
                    "[WARN] progress.toml has exercise {key}, but Cargo.toml does not have one."
                );
            }
        }
    }

    fn save_to_file(&self) {
        let mut content = String::new();
        content.push_str("# This file is auto-generated by `cargo xtask`. DO NOT EDIT!");
        content.push_str("\n\n");
        content.push_str(&format!("version = \"{}\"\n", self.version));
        content.push_str(&format!("current = \"{}\"\n", self.current));

        content.push_str("\n[exercises]\n");
        for (key, progress) in self.exercises.iter() {
            content.push_str(&format!("{} = \"{}\"\n", key, progress));
        }

        fs::write(PROJECT_DIR.join(Self::FILE_NAME), content)
            .expect("Failed to write content to progress.toml");
    }

    fn next_key(&self) -> Option<ExerciseKey> {
        let range = (Bound::Excluded(self.current), Bound::Unbounded);
        self.exercises.range(range).next().map(|(key, _)| *key)
    }

    fn prev_key(&self) -> Option<ExerciseKey> {
        let range = (Bound::Unbounded, Bound::Excluded(self.current));
        self.exercises.range(range).next_back().map(|(key, _)| *key)
    }

    fn status(&self) -> String {
        let mut buf = String::new();
        buf.push_str("🛑 untouched  🚧 partial  ✅️ solved\n");

        let mut it = self.exercises.iter().peekable();
        while let Some((ex, p)) = it.next() {
            buf.push('\n');
            buf.push_str(&format!("  {}", text(*ex, *p)));
            while let Some((key, p)) = it.next_if(|(key, _)| key.digits[0] == ex.digits[0]) {
                buf.push_str(&format!("  {}", text(*key, *p)));
            }
        }

        fn text(key: ExerciseKey, progress: ExerciseStatus) -> String {
            let marker = match progress {
                ExerciseStatus::Untouched => "🛑",
                ExerciseStatus::Partial => "🚧",
                ExerciseStatus::Solved => "✅️",
            };
            format!("{key} {marker}")
        }

        buf
    }
}

fn parse_cargo_toml(
    value: &mut Value,
) -> Result<BTreeMap<ExerciseKey, PathBuf>, toml_span::DeserError> {
    let mut th = TableHelper::new(value)?;
    let bin: Vec<BinTarget> = th.required("bin")?;
    th.finalize(Some(value))?;

    let mut map = BTreeMap::new();
    for tgt in bin {
        map.insert(tgt.name, tgt.path);
    }
    Ok(map)
}

fn main() {
    let mut args = std::env::args().skip(1);

    let cargo_toml_content = fs::read_to_string(PROJECT_DIR.join("Cargo.toml"))
        .expect("Failed to read Cargo.toml into a string buffer.");
    let cargo_toml_value =
        &mut toml_span::parse(&cargo_toml_content).expect("Failed to parse Cargo.toml");
    let bin_paths =
        parse_cargo_toml(cargo_toml_value).expect("Failed to parse Cargo.toml configuration.");

    let version = cargo_toml_value
        .pointer("/package/version")
        .and_then(Value::as_str)
        .expect("package.version must be defined in Cargo.toml");

    let mut progress =
        Progress::load().unwrap_or_else(|| Progress::new(version.to_string(), &bin_paths));

    match args.next().as_deref() {
        Some("help" | "h" | "-h") => eprintln!(
            r###"
    {} xtask [command]

COMMANDS:
    status, s      Show progress (default)
    check, c       Compile and run the current exercise
    next, n, +     Go to the next exercise
    prev, p, -     Go to the previous exercise
    XX             Compile and run XX
    @XX            Go to XX
    all            Compile and run all
    reset          Reset progress
    update         Update progress.toml file
    version, v     Print version of progress.toml
    help, h, -h    Show this message
"###,
            env!("CARGO")
        ),
        Some("status" | "s") | None => {
            println!("\n{}\n", progress.status());
            println!("Current exercise: {}\n", progress.current);
        }
        Some("check" | "c") => {
            let current = progress.current;
            if let Some(path) = bin_paths.get(&current) {
                if run_program(current, path) {
                    *progress.exercises.get_mut(&current).unwrap() = ExerciseStatus::Solved;
                    println!("\nExercise {current} has been solved ✅️\n");
                    if progress.next_key().is_none() {
                        println!("\nYou've reached the end! Congratulations🎉\n");
                    }
                } else {
                    *progress.exercises.get_mut(&current).unwrap() = ExerciseStatus::Partial;
                }
                progress.save_to_file();
            } else {
                errorln(&format!("Exercise {current} does not exist.\n"));
            }
        }
        Some("next" | "n" | "+") => {
            println!("\n{}\n", progress.status());
            if let Some(next_key) = progress.next_key() {
                progress.current = next_key;
                progress.save_to_file();
                println!("Switched to exercise {next_key}.\n");
            } else {
                let (last_key, _) = progress.exercises.last_key_value().unwrap();
                println!("You're on the very last exercise, {last_key}.\n");
            }
        }
        Some("prev" | "p" | "-") => {
            println!("\n{}\n", progress.status());
            if let Some(prev_key) = progress.prev_key() {
                progress.current = prev_key;
                progress.save_to_file();
                println!("Switched to exercise {prev_key}.\n");
            } else {
                let (first_key, _) = progress.exercises.first_key_value().unwrap();
                println!("You're on the very first exercise, {first_key}.\n");
            }
        }
        Some(s) if let Ok(key) = str::parse::<ExerciseKey>(s) => {
            println!();
            if let Some(path) = bin_paths.get(&key) {
                let status = if run_program(key, path) {
                    println!("\nExercise {key} has been solved ✅️\n");
                    ExerciseStatus::Solved
                } else {
                    ExerciseStatus::Partial
                };
                *progress.exercises.get_mut(&key).unwrap() = status;
                progress.save_to_file();
            } else {
                errorln(&format!("Exercise {key} does not exist.\n"));
            }
        }
        Some(s)
            if let mut chars = s.chars()
                && let Some('@') = chars.next()
                && let Ok(key) = str::parse::<ExerciseKey>(chars.as_str()) =>
        {
            println!("\n{}\n", progress.status());

            if bin_paths.contains_key(&key) {
                progress.current = key;
                progress.save_to_file();
                println!("Switched to exercise {key}.\n");
            } else {
                errorln(&format!("Exercise {key} does not exist.\n"));
            }
        }
        Some("all") => {
            for (key, path) in bin_paths {
                let status = if run_program(key, path) {
                    ExerciseStatus::Solved
                } else {
                    ExerciseStatus::Partial
                };
                *progress.exercises.get_mut(&key).unwrap() = status;
                println!();
            }
            progress.save_to_file();
            println!("\n{}\n", progress.status());
            println!("All programs have been checked.\n");
        }
        Some("reset") => {
            progress.reset();
            progress.save_to_file();
            println!("\n{}\n", progress.status());
            println!("All progress have been reset.\n");
        }
        Some("update") => {
            if progress.version.as_str() > version {
                errorln(&format!(
                    "Cargo.toml version ({version}) is older than progress.toml version ({}).",
                    progress.version.as_str()
                ));
            } else {
                if progress.version.as_str() == version {
                    eprintln!(
                        "[INFO] Cargo.toml and progress.toml are already at the same version."
                    );
                    eprintln!("[WARN] Updating exercise entries regardless.");
                }
                let old_version = progress.version.clone();
                progress.update(version, &bin_paths);
                progress.save_to_file();
                println!("\n{}\n", progress.status());
                println!(
                    "progress.toml version has been updated from {old_version} to {version}.\n"
                );
                return;
            };
        }
        Some("version" | "v") => println!("{}", progress.version.as_str()),
        Some("_dev") => check_paths(
            rs_files_in_dir(
                fs::read_dir(PROJECT_DIR.clone()).expect("Failed to read directory information"),
            ),
            rs_files_in_dir(
                fs::read_dir(SOLUTIONS_DIR.clone()).expect("Failed to read directory information"),
            ),
            bin_paths,
        ),
        Some(unk) => eprintln!("Unknown command: {unk}"),
    }

    if progress.version != version {
        eprintln!(
            "[WARN] Cargo.toml version ({version}) and progress.toml version ({}) do not match.",
            progress.version
        );
        eprintln!("[INFO] Try running `cargo xtask update`");
    }
}

fn two_digit_prefix(file_name: &OsStr) -> Option<&str> {
    let prefix = file_name.as_encoded_bytes().first_chunk()?;
    matches!(prefix, [_, _] if prefix.iter().all(u8::is_ascii_digit))
        .then(|| str::from_utf8(prefix).unwrap())
}

fn check_paths(
    exercise_paths: Vec<PathBuf>,
    solution_paths: Vec<PathBuf>,
    target_paths: BTreeMap<ExerciseKey, PathBuf>,
) {
    let mut it_ex = exercise_paths.iter();
    let mut it_sol = solution_paths.iter();
    let mut it_tgt = target_paths.values();
    loop {
        match (it_ex.next(), it_sol.next(), it_tgt.next()) {
            (None, None, None) => break, // done
            (Some(ex), Some(sol), Some(tgt)) => {
                let ex = ex.file_name();
                let sol = sol.file_name();
                if ex == sol && sol == tgt.file_name() {
                    continue;
                } else {
                    panic!(
                        "{ex:?} == {sol:?} && {ex:?} == {:?} is false",
                        tgt.file_name()
                    )
                }
            }
            (None, sol, tgt) => panic!(
                "Exercise file does not exist: {:?}",
                sol.and_then(|s| s.file_name())
                    .or(tgt.and_then(|t| t.file_name()))
                    .unwrap()
            ),
            (ex, None, tgt) => panic!(
                "Solution file does not exist: solutions/{:?}",
                ex.and_then(|s| s.file_name())
                    .or(tgt.and_then(|t| t.file_name()))
                    .unwrap()
            ),
            (ex, sol, None) => {
                let path = ex
                    .and_then(|s| s.file_name())
                    .or(sol.and_then(|s| s.file_name()))
                    .unwrap();
                panic!(
                    concat!(
                        "Binary target does not exist. Add this to Cargo.toml:\n",
                        "[[bin]]\n",
                        "name = \"{}\"\n",
                        "path = {:?}"
                    ),
                    two_digit_prefix(path).unwrap(),
                    path
                );
            }
        }
    }
}

fn run_program(exercise_name: ExerciseKey, target_path: impl AsRef<Path>) -> bool {
    let file_name_osstr = target_path.as_ref().file_name().unwrap();
    let file_name_str = file_name_osstr.to_string_lossy();
    println!("╭───────────{}──╮", "─".repeat(file_name_str.len()));
    println!("│  Program: {}  │", file_name_str);
    println!("╰───────────{}──╯", "─".repeat(file_name_str.len()));

    let solution_path = SOLUTIONS_DIR.join(file_name_osstr);
    let output_answer = std::process::Command::new("rust-script")
        .args([solution_path.as_os_str()])
        .env("CLICOLOR_FORCE", "1")
        .output()
        .expect("Install rust-script by running: `cargo install rust-script`");

    let output = std::process::Command::new(env!("CARGO"))
        .args([
            OsStr::new("run"),
            OsStr::new("--quiet"),
            OsStr::new("--bin"),
            OsStr::new(&exercise_name.to_string()),
        ])
        .env("CLICOLOR_FORCE", "1")
        .output()
        .expect("");

    let stdout_diff = similar::TextDiff::from_lines(
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output_answer.stdout),
    );
    println!("\n  [OUTPUT]\n");
    for change in stdout_diff.iter_all_changes() {
        match change.tag() {
            similar::ChangeTag::Insert => print!("{DIM}+|{change}{RESET}"),
            similar::ChangeTag::Delete => print!("{RED}-|{change}{RESET}"),
            similar::ChangeTag::Equal => print!("{GREEN}{change}{RESET}"),
        }
    }
    let stderr_diff = similar::TextDiff::from_lines(
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output_answer.stderr),
    );
    println!("\n  [ERROR]\n");
    for change in stderr_diff.iter_all_changes() {
        match change.tag() {
            similar::ChangeTag::Insert => print!("+|{change}"),
            similar::ChangeTag::Delete => print!("-|{change}"),
            similar::ChangeTag::Equal => print!("{change}"),
        }
    }

    use similar::DiffTag::Equal;
    output.status == output_answer.status
        && stdout_diff.ops().iter().all(|op| op.tag() == Equal)
        && stderr_diff.ops().iter().all(|op| op.tag() == Equal)
}
