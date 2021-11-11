mod lisp;
mod parser;

fn main() {
    let inp = "(+ (+ 2 3) (+ 4 5 (+ 6 7))".to_string();
    let whitespaced = inp.replace("(", " ( ").replace(")", " ) ");
    let mut iter = whitespaced.split_whitespace();

    let parsed = parser::parse(&mut iter);
    let mut env = lisp::LispEnv::default();
    let expr = parsed.to_lispexpr().extract();

    println!("{:?}", expr);
    println!("{:?}", expr.eval(&mut env).unwrap());
}
