use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use cargo_metadata::Target;

static PROJECT_DIR: LazyLock<&Path> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("Could not locate project root")
});
static SOLUTOINS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PROJECT_DIR.join("solutions"));

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

fn main() {
    let cargo_toml = PROJECT_DIR.join("Cargo.toml");
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(cargo_toml)
        .exec()
        .expect("Failed to retrieve cargo metadata");
    let cargo_bin_targets: Vec<_> = metadata
        .root_package()
        .expect("Failed to find the root package")
        .targets
        .iter()
        .filter(|tgt| tgt.is_bin())
        .collect();

    check_paths(
        rs_files_in_dir(
            fs::read_dir(PROJECT_DIR.clone()).expect("Failed to read directory information"),
        ),
        rs_files_in_dir(
            fs::read_dir(SOLUTOINS_DIR.clone()).expect("Failed to read directory information"),
        ),
        cargo_bin_targets,
    );
}

fn two_digit_prefix(file_name: &str) -> Option<&str> {
    let prefix = file_name.as_bytes().first_chunk()?;
    matches!(prefix, [_, _] if prefix.iter().all(u8::is_ascii_digit))
        .then(|| str::from_utf8(prefix).unwrap())
}

fn check_paths(
    exercise_paths: Vec<PathBuf>,
    solution_paths: Vec<PathBuf>,
    bin_targets: Vec<&Target>,
) {
    for tgt in bin_targets.iter() {
        assert!(tgt.is_bin(), "{} must be a binary target.", tgt.name);

        let Some(prefix) = two_digit_prefix(tgt.src_path.file_stem().unwrap()) else {
            panic!("Binary path {} should start with 2 digits", tgt.src_path);
        };
        assert_eq!(
            tgt.name, prefix,
            "Binary path \"{}\" must start with prefix {prefix}.",
            tgt.name
        );
    }

    // fn path2name(path: Option<&PathBuf>) -> Option<&str> {
    //     path.and_then(|s| s.file_name().and_then(|s| s.to_str()))
    // }
    // fn target2name(target: Option<&Target>) -> Option<&str> {
    //     target.and_then(|t| t.src_path.file_name())
    // }

    let mut it_ex = exercise_paths.iter();
    let mut it_sol = solution_paths.iter();
    let mut it_tgt = bin_targets.iter();
    loop {
        match (it_ex.next(), it_sol.next(), it_tgt.next()) {
            (None, None, None) => break, // done
            (Some(ex), Some(sol), Some(tgt)) => {
                let ex = ex.file_name().and_then(OsStr::to_str);
                let sol = sol.file_name().and_then(OsStr::to_str);
                if ex == sol && sol == tgt.src_path.file_name() {
                    continue;
                } else {
                    panic!(
                        "{:?} == {:?} && {1:?} == {:?} is false",
                        ex,
                        sol,
                        tgt.src_path.file_name()
                    )
                }
            }
            (None, sol, tgt) => panic!(
                "Exercise file does not exist: {}",
                sol.and_then(|s| s.file_name().and_then(|s| s.to_str()))
                    .or(tgt.and_then(|t| t.src_path.file_name()))
                    .unwrap()
            ),
            (ex, None, tgt) => panic!(
                "Solution file does not exist: solutions/{}",
                ex.and_then(|s| s.file_name().and_then(OsStr::to_str))
                    .or(tgt.and_then(|t| t.src_path.file_name()))
                    .unwrap()
            ),
            (ex, sol, None) => {
                let path = ex
                    .and_then(|s| s.file_name().and_then(|s| s.to_str()))
                    .or(sol.and_then(|s| s.file_name().and_then(|s| s.to_str())))
                    .unwrap();
                panic!(
                    concat!(
                        "Binary target does not exist. Add this to Cargo.toml:\n",
                        "[[bin]]\n",
                        "name = \"{}\"\n",
                        "path = \"{}\""
                    ),
                    two_digit_prefix(path).unwrap(),
                    path
                );
            }
        }
    }
}

#[expect(dead_code)]
fn run_programs() {
    for solution_path in rs_files_in_dir(
        fs::read_dir(SOLUTOINS_DIR.clone()).expect("Failed to read directory information"),
    ) {
        println!("{:?}", solution_path);
        let output = std::process::Command::new("rust-script")
            // .args([OsStr::new("run"), solution_path.as_os_str()])
            .args([solution_path.as_os_str()])
            .env("CLICOLOR_FORCE", "1")
            .output()
            .expect("");

        println!("{:?}", output.status);
        println!(
            "[stdout]\n{}",
            String::from_utf8_lossy(output.stdout.as_slice()),
            // String::from_utf8(output.stdout).expect("Not UTF-8")
        );
        println!(
            "[stderr]\n{}",
            String::from_utf8_lossy(output.stderr.as_slice()),
            // String::from_utf8(output.stderr).expect("Not UTF-8")
        );

        // let exercise_path = PROJECT_DIR.join(solution_path.file_name().unwrap());
        let exercise_name: &[_; 2] = solution_path
            .file_stem()
            .unwrap()
            .as_encoded_bytes()
            .first_chunk()
            .filter(|s| s[0].is_ascii_digit() && s[1].is_ascii_digit())
            .expect("should start with 2 digit exercise name");

        let output = std::process::Command::new(env!("CARGO"))
            .args([
                OsStr::new("check"),
                OsStr::new("--bin"),
                OsStr::new(str::from_utf8(exercise_name).unwrap()),
            ])
            .env("CLICOLOR_FORCE", "1")
            .output()
            .expect("");

        println!("{:?}", output.status);
        println!(
            "[stdout]\n{}",
            String::from_utf8_lossy(output.stdout.as_slice()),
            // String::from_utf8(output.stdout).expect("Not UTF-8")
        );
        println!(
            "[stderr]\n{}",
            String::from_utf8_lossy(output.stderr.as_slice()),
            // String::from_utf8(output.stderr).expect("Not UTF-8")
        );
    }
    for exercise_path in
        rs_files_in_dir(fs::read_dir(*PROJECT_DIR).expect("Failed to read directory information"))
    {
        println!("{:?}", exercise_path);
    }
}

#[cfg(test)]
mod test {}
