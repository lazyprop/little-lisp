pub enum AstNode {
    Leaf(String),
    Node(Vec<AstNode>),
}

impl AstNode {
    pub fn print(&self) {
        match self {
            AstNode::Leaf(s) => print!("{} ", s),
            AstNode::Node(v) => { print!("("); for n in v {
                n.print();
            } print!(")"); },
        }
    }

    fn push(&mut self, val: AstNode) {
        match self {
            AstNode::Node(v) => v.push(val),
            _ => (),
        }
    }
}

type ParseErr = String;

pub fn parse(iter: &mut Iterator<Item = &str>) -> AstNode  {
    let mut ast = AstNode::Node(Vec::new());
    loop {
        match iter.next() {
            Some(token) => match token {
                "(" => ast.push(parse(iter)),
                ")" => break,
                _ => ast.push(AstNode::Leaf(token.to_string())),
            },
            None => break,
        }
    }
    ast
}
