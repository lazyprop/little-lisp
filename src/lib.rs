use std::collections::HashMap;

type ArgsType = Vec<LispExpr>; // all lisp function take a list of arguments
type ReturnType = Result<LispExpr, LispErr>; // all lisp functions return this type

#[derive(Clone, Debug, PartialEq)]
struct LispFunc {
    func: fn(ArgsType) -> ReturnType,
    arity: usize, // number of arguments
    inf_args: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum LispExpr {
    Symbol(String),
    Integer(i64),
    List(Vec<LispExpr>),
    Func(LispFunc)
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

    fn parse_fn(&self) -> Result<LispFunc, LispErr> {
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
            // TODO does slicing here create a copy?
            let args = list[1..].iter()
                .map(|a| eval(a, env))
                .collect::<Result<Vec<_>, _>>()?;

            if !f.inf_args && f.arity != args.len() {
                return Err(LispErr::ArityMismatch)
            }

            // recursively evaluate until the return value is not an atom
            eval(&(f.func)(args)?, env) // TODO who owns the value here?
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

        use LispExpr::*;

        // addition
        env.insert(
            "+".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    // TODO too many copies?
                    // TODO find a better way to do this
                    let ans = args.iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(Integer(ans))
                },
                inf_args: true,
                arity: 0,
            })
        );

        // subtraction
        env.insert(
            "-".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    let first = args[0].parse_int()?;
                    let ans: i64 = args[1..].iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(Integer(first - ans))
                },
                inf_args: true,
                arity: 0,
            })
        );

        // multiplication
        env.insert(
            "*".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    let ans = args.iter()
                        .map(|a| a.parse_int() )
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .product();
                    Ok(Integer(ans))
                },
                inf_args: true,
                arity: 0,
            })
        );

        env.insert(
            "square".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    // TODO remove these clones
                    // TODO should this be wrapped in an Ok?
                    Ok(List(vec![Symbol("*".to_string()),
                                args[0].clone(), args[0].clone()]))
                },
                inf_args: false,
                arity: 1,
            })
        );

        env.insert(
            // always fails
            "bad-func".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    Ok(List(vec![Symbol("not-defined".to_string())]))
                },
            inf_args: false,
            arity: 0
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
    fn eval_arithmetic() {
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

    #[test]
    fn eval_functions() {
        let mut env = LispEnv::default();

        let expr = List(vec![Symbol("square".to_string()), Integer(5)]);
        assert_eq!(eval(&expr, &mut env).unwrap(), Integer(25));

        let expr = List(vec![Symbol("bad-func".to_string())]);
        assert!(eval(&expr, &mut env).is_err());

        let expr = List(vec![Symbol("square".to_string()),
                             Integer(3),
                             Integer(4)]);

        // TODO write better tests
        assert!(match eval(&expr, &mut env) {
                    Err(LispErr::ArityMismatch) => true,
                    _ => false,
        });
    }
}
