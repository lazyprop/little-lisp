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
                //println!("Input: {}", line);
                rl.add_history_entry(line.as_str());

                let whitespaced = line.replace("(", " ( ").replace(")", " ) ");
                let mut iter = whitespaced.split_whitespace();
                let parsed = parser::parse(&mut iter);
                let expr = parsed.to_lispexpr().extract_first();

                //println!("Parsed Expression: {:?}\n", expr);
                let res = expr.eval(&mut env);
                match res {
                    Ok(val) => match val {
                        LispExpr::Integer(i) => println!("{}", i),
                        LispExpr::Bool(b) => println!("{}", b),
                        LispExpr::Null => (),
                        _ => println!("void"),
                    },
                    Err(e) => println!("error: {:?}", e),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
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
        let expr = parsed.to_lispexpr().extract_first();

        println!("{:?}", expr);
        println!("{:?}", expr.eval(&mut env).unwrap());
    }

    #[test]
    fn main_test() {
        let inp = "(define (add a b) (+ a b))".to_string();
        let whitespaced = inp.replace("(", " ( ").replace(")", " ) ");
        let mut iter = whitespaced.split_whitespace();

        let parsed = parser::parse(&mut iter);
        let mut env = lisp::LispEnv::default();
        let expr = parsed.to_lispexpr().extract_first();

        println!("{:?}", expr);
        println!("{:?}", expr.eval(&mut env).unwrap());
    }
}
