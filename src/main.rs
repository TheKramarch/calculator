use std::env;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use calculator::{evaluate, format_value};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        repl();
        return ExitCode::SUCCESS;
    }

    let expression = args.join(" ");
    match evaluate(&expression) {
        Ok(value) => {
            println!("{}", format_value(value));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Ошибка: {error}");
            ExitCode::FAILURE
        }
    }
}

fn repl() {
    println!("Калькулятор. Введите выражение (exit — выход).");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Ошибка чтения ввода: {error}");
                break;
            }
        };

        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        match evaluate(input) {
            Ok(value) => println!("{}", format_value(value)),
            Err(error) => eprintln!("Ошибка: {error}"),
        }
    }

    let _ = io::stdout().flush();
}
