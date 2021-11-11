use crate::lisp::LispExpr;

pub enum AstNode {
    Leaf(LispExpr),
    Node(Vec<AstNode>),
}

impl AstNode {
    #[allow(dead_code)]
    pub fn print(&self) {
        match self {
            AstNode::Leaf(expr) => expr.print(),
            AstNode::Node(v) => {
                print!("(");
                for n in v {
                    n.print();
                }
                print!(")");
            }
        }
    }

    fn push(&mut self, val: AstNode) {
        // TODO handle the case when it might not be a Node
        // TODO prevent that case from happening altogether
        #[allow(clippy::single_match)]
        match self {
            AstNode::Node(v) => v.push(val),
            _ => (),
        }
    }

    // TODO this is redundant.
    // TODO i should directly build the AST as a LispExpr
    pub fn to_lispexpr(&self) -> LispExpr {
        match self {
            AstNode::Leaf(e) => e.clone(),
            AstNode::Node(v) => {
                LispExpr::List(v.iter().map(|e| e.to_lispexpr()).collect::<Vec<_>>())
            }
        }
    }
}

#[allow(dead_code)]
type ParseErr = String;

fn to_lispexpr(token: &str) -> LispExpr {
    match token.parse::<i64>() {
        Ok(num) => LispExpr::Integer(num),
        Err(_) => LispExpr::Symbol(token.to_string()),
    }
}

pub fn parse(iter: &mut dyn Iterator<Item = &str>) -> AstNode {
    let mut ast = AstNode::Node(Vec::new());
    while let Some(token) = iter.next() {
        match token {
            "(" => ast.push(parse(iter)),
            ")" => break,
            _ => ast.push(AstNode::Leaf(to_lispexpr(token))),
        }
    }
    ast
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        assert!(true);
    }
}
