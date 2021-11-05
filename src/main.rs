mod parser;

fn main() {
    let inp = "(+ (+ 2 3) (+ 4 5 (+ 6 7))".to_string();
    let whitespaced = inp.replace("(", " ( ").replace(")", " ) ");
    let mut iter = whitespaced.split_whitespace();

    let parsed = parser::parse(&mut iter);
    parsed.print();
}
