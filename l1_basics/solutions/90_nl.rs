// このエクササイズでは nl というコマンドラインツールを作ります。
//
// すでにファイル入出力の大部分は出来ています。
// process_lines() の一部を変えてプログラムを完成させてください。

use std::io;

fn main() -> io::Result<()> {
    // args はこのプログラムを実行するときに受け取る引数です。
    let args: Vec<String> = std::env::args().collect();
    App::new(&args).run()
}

struct App {
    file_path: Option<String>,
}

impl App {
    fn new(args: &[String]) -> Self {
        let mut file_path: Option<String> = None;

        // この while 文ではプログラムの引数を解析しています。
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
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

        App { file_path }
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
            let line = line?; // 1行読み出す毎に入出力エラーの可能性あり

            // nl は空行以外の行番号を数えます。
            if line.is_empty() {
                println!("{line_number:>6}\t{line}");
                line_number += 1;
            } else {
                println!("      \t{line}");
            }
        }
        Ok(())
    }
}
