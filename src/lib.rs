use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum LispExpr {
    Symbol(String),
    Integer(i64),
    List(Vec<LispExpr>),
    Func(fn(Vec<LispExpr>) -> LispExpr),
}

fn eval(expr: &LispExpr, env: &mut LispEnv) -> Result<LispExpr, String> {
    match expr {
        LispExpr::Symbol(s) => match env.data.get(s) {
            Some(x) => Ok(x.clone()),
            None => Err(String::from("Symbol not found in environment")),
        },
        LispExpr::Integer(_) => Ok(expr.clone()),
        LispExpr::List(list) => {
            let first = match eval(&list[0], env) {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            match first {
                LispExpr::Func(f) => {
                    let mut args = Vec::<LispExpr>::new();
                    for e in list[1..].iter() {
                        match eval(e, env) {
                            Ok(val) => args.push(val),
                            Err(x) => return Err(x),
                        }
                    }

                    Ok(f(args))
                },
                _ => Err(String::from("First element not a function")),
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

    fn add(args: Vec<LispExpr>) -> LispExpr {
        let mut ans: i64 = 0;
        for a in args {
            match a {
                Integer(n) => ans += n,
                _ => ()
            }
        }
        Integer(ans)
    }

    #[test]
    fn eval_test() {
        let mut env = LispEnv::default();
        //env.insert(String::from("+"), Func(add));

        let expr = List(vec![Symbol(String::from("+")),
                    List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
                    Integer(4)]);
        match eval(&expr, &mut env) {
            Ok(val) => assert_eq!(val, Integer(12)),
            Err(_) => assert!(false),
        }

        let expr = List(vec![Symbol(String::from("+")),
                    List(vec![Symbol(String::from("-")), Integer(3), Integer(5)]),
                    Integer(4)]);
        match eval(&expr, &mut env) {
            Ok(val) => assert_eq!(val, Integer(-4)),
            Err(_) => assert!(false),
        }

        let expr = List(vec![Symbol(String::from("*")),
                    List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
                    Integer(4)]);
        match eval(&expr, &mut env) {
            Ok(val) => assert_eq!(val, Integer(32)),
            Err(_) => assert!(false),
        }
    }
}
