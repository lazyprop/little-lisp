mod lisp;
mod parser;

use lisp::{LispExpr, LispEnv, LispErr};

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
                println!("Line: {}", line);
                rl.add_history_entry(line.as_str());

                let whitespaced = line.replace("(", " ( ").replace(")", " ) ");
                let mut iter = whitespaced.split_whitespace();
                let parsed = parser::parse(&mut iter);
                let expr = parsed.to_lispexpr().extract();

                println!("{:?}", expr);
                println!("{:?}", expr.eval(&mut env).unwrap());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history("history.txt").unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let inp = "(+ (+ 2 3) (+ 4 5 (+ 6 7))".to_string();
        let whitespaced = inp.replace("(", " ( ").replace(")", " ) ");
        let mut iter = whitespaced.split_whitespace();

        let parsed = parser::parse(&mut iter);
        let mut env = lisp::LispEnv::default();
        let expr = parsed.to_lispexpr().extract();

        println!("{:?}", expr);
        println!("{:?}", expr.eval(&mut env).unwrap());
    }
}

