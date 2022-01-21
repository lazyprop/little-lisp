mod lisp;
mod parser;

use lisp::{LispEnv, LispExpr};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut env = LispEnv::default();

    // `()` can be used when no completer is required
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

                let res = expr.eval(&mut env);
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
