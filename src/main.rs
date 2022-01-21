mod lisp;
mod parser;

use lisp::{LispEnv};

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;

fn eval_string(line: String, env: &mut LispEnv) {
    let whitespaced = line.replace("(", " ( ").replace(")", " ) ");
    let mut iter = whitespaced.split_whitespace();
    let parsed = parser::parse(&mut iter);
    let list = parsed.to_lispexpr().extract_list()
        .expect("input not a list");

    for expr in list {
        let res = expr.eval(env);
        match res {
            Ok(val) => {
                if let Some(s) = val.to_string() {
                    println!("{}", s);
                }
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
}

fn repl(env: &mut LispEnv) {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                eval_string(line, env);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn eval_file(filename: &str, env: &mut LispEnv) {
    let contents = fs::read_to_string(filename)
        .expect(format!("failed to read {}", filename).as_str());
    eval_string(contents, env);
}

fn main() {
    let mut env = LispEnv::new();

    repl(&mut env);
}
