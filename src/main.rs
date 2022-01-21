mod lisp;
mod parser;

use lisp::{LispEnv};

use rustyline::error::ReadlineError;
use rustyline::Editor;

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

                let whitespaced = line.replace("(", " ( ").replace(")", " ) ");
                let mut iter = whitespaced.split_whitespace();
                let parsed = parser::parse(&mut iter);
                let expr = parsed.to_lispexpr().extract_first();

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

fn main() {
    let mut env = LispEnv::default();

    repl(&mut env);
}
