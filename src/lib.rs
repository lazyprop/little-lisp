use std::collections::HashMap;

// all lisp functions return this type
type ReturnType = Result<LispExpr, LispErr>;

#[derive(Clone, Debug, PartialEq)]
enum LispExpr {
    Symbol(String),
    Integer(i64),
    List(Vec<LispExpr>),
    Func(fn(Vec<LispExpr>) -> ReturnType),
}


impl LispExpr {
    fn parse_symbol(&self) -> Result<String, LispErr> {
        match self {
            LispExpr::Symbol(s) => Ok(s.clone()),
            _ => Err(LispErr::TypeError),
        }
    }
    fn parse_int(&self) -> Result<i64, LispErr> {
        match self {
            LispExpr::Integer(i) => Ok(i.clone()),
            _ => Err(LispErr::TypeError),
        }
    }

    fn parse_fn(&self) -> Result<fn(Vec<LispExpr>) -> ReturnType, LispErr> {
        match self {
            LispExpr::Func(f) => Ok(f.clone()),
            _ => Err(LispErr::TypeError),
        }
    }
}

#[derive(Debug)]
enum LispErr {
    ArityMismatch,
    NameError,
    TypeError,
}

fn eval(expr: &LispExpr, env: &mut LispEnv) -> ReturnType {
    match expr {
        LispExpr::Symbol(s) => match env.data.get(s) {
            Some(e) => Ok(e.clone()),
            None => Err(LispErr::NameError),
        },
        LispExpr::Integer(_) => Ok(expr.clone()),
        LispExpr::List(list) => {
            let f = eval(&list[0], env)?.parse_fn()?;
            let args = list[1..].iter()
                .map(|a| eval(a, env))
                .collect::<Result<Vec<_>, _>>()?;
            f(args)
        },
        LispExpr::Func(_) => Ok(expr.clone()),
    }
}


struct LispEnv {
    data: HashMap<String, LispExpr>,
}

impl LispEnv {
    fn new() -> LispEnv {
        LispEnv { data: HashMap::new() }
    }

    fn default() -> LispEnv {
        let mut env = LispEnv::new();

        env.insert(
            String::from("+"),
            LispExpr::Func(
                |args: Vec<LispExpr>| -> ReturnType {
                    // TODO too many copies?
                    // TODO find a better way to do this
                    let ans = args.iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(LispExpr::Integer(ans))
                })
            );

        env.insert(
            String::from("-"),
            LispExpr::Func(
                |args: Vec<LispExpr>| -> ReturnType {
                    let first = args[0].parse_int()?;
                    let ans: i64 = args.iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(LispExpr::Integer(2i64*first - ans))
                })
            );

        env.insert(
            String::from("*"),
            LispExpr::Func(
                |args: Vec<LispExpr>| -> ReturnType {
                    let ans: i64 = args.iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .product();
                    Ok(LispExpr::Integer(ans))
                })
            );


        env
    }

    fn insert(&mut self, name: String, expr: LispExpr) {
        self.data.insert(name, expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LispExpr::*;

    #[test]
    fn eval_test() {
        let mut env = LispEnv::default();

        let expr = List(vec![Symbol(String::from("+")),
                    List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
                    Integer(4)]);
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(12));

        let expr = List(vec![Symbol(String::from("+")),
                    List(vec![Symbol(String::from("-")), Integer(3), Integer(5)]),
                    Integer(4)]);
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(2));

        let expr = List(vec![Symbol(String::from("*")),
                    List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
                    Integer(4)]);
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(32));
    }
}
