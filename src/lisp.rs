use std::collections::HashMap;

type ArgsType = Vec<LispExpr>; // all lisp function take a list of arguments
type ReturnType = Result<LispExpr, LispErr>; // all lisp functions return this type

#[derive(Clone, Debug)]
pub struct LispFunc {
    params: Vec<LispExpr>,
    body: LispExpr,
}

impl LispFunc {
    fn call(self: &LispFunc, args: Vec<LispExpr>, env: &mut LispEnv) -> ReturnType {
        env.new_frame();
        for (i, x) in self.params.iter().enumerate() {
            let a = args[i].clone().eval(env)?;
            env.insert(x.extract_symbol()?, a);
        }
        let res = self.body.eval(env);
        env.pop_frame();
        res
    }
}

#[derive(Clone, Debug)]
pub enum LispExpr {
    Symbol(String),
    Integer(i64),
    List(Vec<LispExpr>),
    Bool(bool),
    Func(Box<LispFunc>),
    Null,
}

impl LispExpr {
    #[allow(dead_code)]
    fn extract_list(&self) -> Result<Vec<LispExpr>, LispErr> {
        match self {
            LispExpr::List(l) => Ok(l.clone()),
            _ => Err(LispErr::TypeError("expected list".to_string())),
        }
    }

    fn extract_symbol(&self) -> Result<String, LispErr> {
        match self {
            LispExpr::Symbol(s) => Ok(s.clone()),
            _ => Err(LispErr::TypeError("expected symbol".to_string())),
        }
    }

    #[allow(dead_code)]
    fn extract_int(&self) -> Result<i64, LispErr> {
        match self {
            LispExpr::Integer(n) => Ok(*n),
            _ => Err(LispErr::TypeError("expected integer".to_string())),
        }
    }

    #[allow(dead_code)]
    fn extract_bool(&self) -> Result<bool, LispErr> {
        match self {
            LispExpr::Bool(b) => Ok(*b),
            _ => Err(LispErr::TypeError("expected bool".to_string())),
        }
    }

    fn extract_fn(&self) -> Result<Box<LispFunc>, LispErr> {
        match self {
            LispExpr::Func(f) => Ok(f.clone()),
            _ => Err(LispErr::TypeError("expected function".to_string())),
        }
    }

    // TODO this shouldn't be required
    // This is needed because the parser creates a List of LispExprs, but it
    // it cannot be evaluated directly because it's not in the proper format
    // (the first element is not a Func)
    pub fn extract_first(&self) -> LispExpr {
        match self {
            LispExpr::List(v) => v[0].clone(),
            _ => self.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        match self {
            LispExpr::Symbol(s) => print!("Symbol: {},", s),
            LispExpr::Integer(n) => print!("Integer: {},", n),
            _ => (),
        }
    }

    pub fn eval(&self, env: &mut LispEnv) -> ReturnType {
        match self {
            LispExpr::Symbol(s) => match env.get(s) {
                Some(e) => Ok(e),
                None => Err(LispErr::NameError),
            },
            LispExpr::Integer(_) => Ok(self.clone()),
            LispExpr::List(list) => {
                if list.is_empty() {
                    return Ok(LispExpr::Null);
                }

                // handle special forms
                match &list[0].extract_symbol()?.as_str() {
                    &"define" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }

                        match &list[1] {
                            // we're just defining a symbol
                            LispExpr::Symbol(s) => {
                                env.insert(s.clone(), list[2].clone());
                                return Ok(LispExpr::Null);
                            }
                            // we're defining a procedure
                            LispExpr::List(lst) => {
                                let fname = lst[0].extract_symbol()?;
                                let params = &lst[1..];
                                let body = &list[2];

                                env.insert(
                                    fname,
                                    LispExpr::Func(Box::new(LispFunc {
                                        params: params.to_vec(),
                                        body: body.clone(),
                                    })),
                                );

                                return Ok(LispExpr::Null);
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }

                let func = &list[0].eval(env)?.extract_fn()?;
                let args = &list[1..];
                //println!("{:?}", args);

                if args.len() != func.params.len() {
                    return Err(LispErr::ArityMismatch);
                }

                // recursively evaluate until the return value is not an atom
                func.call(args.to_vec(), env)
            }
            LispExpr::Func(_) => Ok(self.clone()),
            LispExpr::Bool(_) => Ok(self.clone()),
            LispExpr::Null => Ok(self.clone()),
        }
    }
}

#[derive(Debug)]
pub enum LispErr {
    ArityMismatch,
    NameError,
    TypeError(String),
}

#[derive(Debug)]
pub struct LispEnv {
    stack: Vec<HashMap<String, LispExpr>>,
}

impl LispEnv {
    fn new() -> LispEnv {
        LispEnv {
            stack: vec![HashMap::new()],
        }
    }

    pub fn default() -> LispEnv {
        let mut env = LispEnv::new();
        let a = "a".to_string();
        let b = "b".to_string();
        env.insert(
            "first".to_string(),
            LispExpr::Func(Box::new(LispFunc {
                params: vec![LispExpr::Symbol(a.clone()), LispExpr::Symbol(b)],
                body: LispExpr::Symbol(a),
            })),
        );
        env
    }

    fn new_frame(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn pop_frame(&mut self) {
        self.stack.pop();
    }

    fn insert(&mut self, name: String, expr: LispExpr) {
        self.stack.last_mut().unwrap().insert(name, expr);
    }

    fn get(&self, name: &str) -> Option<LispExpr> {
        for i in (0..self.stack.len()).rev() {
            match self.stack[i].get(name) {
                Some(res) => {
                    if let LispExpr::Symbol(new_name) = res {
                        return self.get(new_name);
                    } else {
                        return Some(res.clone());
                    }
                }
                None => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LispExpr::*;

    #[test]
    fn eval_arithmetic() {
        let mut env = LispEnv::default();

        let expr = List(vec![
            Symbol(String::from("+")),
            List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
            Integer(4),
        ]);
        assert_eq!(expr.eval(&mut env).unwrap(), Integer(12));

        let expr = List(vec![
            Symbol(String::from("+")),
            List(vec![Symbol(String::from("-")), Integer(3), Integer(5)]),
            Integer(4),
        ]);
        assert_eq!(expr.eval(&mut env).unwrap(), Integer(2));

        let expr = List(vec![
            Symbol(String::from("*")),
            List(vec![Symbol(String::from("+")), Integer(3), Integer(5)]),
            Integer(4),
        ]);
        assert_eq!(expr.eval(&mut env).unwrap(), Integer(32));
    }

    #[test]
    fn eval_functions() {
        let mut env = LispEnv::default();

        let expr = List(vec![Symbol("square".to_string()), Integer(5)]);
        assert_eq!(expr.eval(&mut env).unwrap(), Integer(25));

        let expr = List(vec![Symbol("bad-func".to_string())]);
        assert!(expr.eval(&mut env).is_err());

        let expr = List(vec![Symbol("square".to_string()), Integer(3), Integer(4)]);

        // TODO write better tests
        assert!(match expr.eval(&mut env) {
            Err(LispErr::ArityMismatch) => true,
            _ => false,
        });
    }

    #[test]
    fn special_forms() {
        let mut env = LispEnv::default();
        let expr = List(vec![
            Symbol("define".to_string()),
            Symbol("add".to_string()),
            Symbol("+".to_string()),
        ]);
        expr.eval(&mut env).unwrap();
    }
}
