use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum LispExpr {
    Symbol(String),
    Integer(i64),
    List(Vec<LispExpr>),
    Func(fn(Vec<LispExpr>) -> LispExpr), // TODO should Func return Result?
}

#[derive(Debug)]
enum LispErr {
    ArityMismatch,
    NameError,
    NotCallable,
}

fn eval(expr: &LispExpr, env: &mut LispEnv) -> Result<LispExpr, LispErr> {
    match expr {
        LispExpr::Symbol(s) => match env.data.get(s) {
            Some(e) => Ok(e.clone()),
            None => Err(LispErr::NameError),
        },
        LispExpr::Integer(_) => Ok(expr.clone()),
        LispExpr::List(list) => {
            if let LispExpr::Func(f) = eval(&list[0], env)? {
                let args = list[1..].iter()
                    .map(|a| eval(a, env))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(f(args))
            } else  {
                Err(LispErr::NotCallable)
            }
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
                |args: Vec<LispExpr>| -> LispExpr {
                    let mut ans = 0;
                    for a in args {
                        match a {
                            LispExpr::Integer(n) => ans += n,
                            _ => ()
                        }
                    }
                    LispExpr::Integer(ans)
                })
            );

        env.insert(
            String::from("-"),
            LispExpr::Func(
                |args: Vec<LispExpr>| -> LispExpr {
                    let mut ans = 0;
                    for a in args {
                        match a {
                            LispExpr::Integer(n) => ans -= n,
                            _ => ()
                        }
                    }
                    LispExpr::Integer(ans)
                })
            );

        env.insert(
            String::from("*"),
            LispExpr::Func(
                |args: Vec<LispExpr>| -> LispExpr {
                    let mut ans = 1;
                    for a in args {
                        match a {
                            LispExpr::Integer(n) => ans *= n,
                            _ => ()
                        }
                    }
                    LispExpr::Integer(ans)
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
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(-4));

        let expr = List(vec![Symbol(String::from("*")),
                    List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
                    Integer(4)]);
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(32));
    }
}
