// nl はデフォルトでは空行以外の行番号しか数えません。
// このプログラムでは全行数えるオプションと全く数えないオプションを追加しました。

use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    App::new(&args).run()
}

struct App {
    style: NumberingStyle,
    file_path: Option<String>,
}

impl App {
    fn new(args: &[String]) -> Self {
        let mut style = NumberingStyle::NonEmpty;
        let mut file_path: Option<String> = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-b" | "--body-numbering" => {
                    if i + 1 < args.len() {
                        style = match args[i + 1].as_str().parse() {
                            Ok(style) => style,
                            Err(err) => {
                                eprintln!("nl: {err}");
                                std::process::exit(1);
                            }
                        };
                        i += 2;
                    } else {
                        eprintln!("nl: option requires an argument -- 'b'");
                        std::process::exit(1);
                    }
                }
                arg if !arg.starts_with('-') => {
                    file_path = Some(arg.to_string());
                    i += 1;
                }
                unk => {
                    eprintln!("nl: invalid option -- '{unk}'");
                    std::process::exit(1);
                }
            }
        }

        App { style, file_path }
    }

    fn run(&self) -> io::Result<()> {
        match &self.file_path {
            Some(path) => {
                let file = std::fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                self.process_lines(reader)
            }
            None => {
                let stdin = io::stdin();
                let reader = stdin.lock();
                self.process_lines(reader)
            }
        }
    }

    fn process_lines(&self, reader: impl io::BufRead) -> io::Result<()> {
        let mut line_number = 1;
        for line in reader.lines() {
            let line = line?;

            if self.style.should_number(&line) {
                println!("{line_number:>6}\t{line}");
                line_number += 1;
            } else {
                println!("      \t{line}");
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum NumberingStyle {
    All,      // 'a'  Number all lines
    NonEmpty, // 't'  Number only non-empty lines (Default)
    None,     // 'n'  Do not number any lines
}

impl NumberingStyle {
    fn should_number(&self, line: &str) -> bool {
        // NumberingStyleのコメントを参考にmatch式を実装しましょう。
        match self {
            // ...
        }
    }
}

// FromStrトレイトは、impl FromStr for T を実装すると、
// str::parse() -> NumberingStyle が自動的に実装されます。
impl std::str::FromStr for NumberingStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NumberingStyleのコメントを参考にmatch式を実装しましょう。
        match s {
            other => Err(format!("invalid body numbering style: {}", other)),
        }
    }
}
