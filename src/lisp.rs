use std::collections::HashMap;

type ReturnType = Result<LispExpr, LispErr>; // all lisp functions return this type

#[derive(Clone, Debug)]
pub struct LispFunc {
    params: Vec<LispExpr>,
    body: Vec<LispExpr>,
}

impl LispFunc {
    fn call(self: &LispFunc, args: Vec<LispExpr>, env: &mut LispEnv) -> ReturnType {
        env.new_frame();
        for (i, x) in self.params.iter().enumerate() {
            let a = args[i].clone().eval(env)?;
            env.insert(x.extract_symbol()?, a);
        }
        for i in 0..self.body.len() - 1 {
            self.body[i].eval(env)?;
        }
        let res = self.body.last().expect("function body is empty").eval(env);
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
    Cons(Box<LispExpr>, Box<LispExpr>),
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

    pub fn print(&self) {
        match self {
            LispExpr::Symbol(s) => print!("Symbol: {},", s),
            LispExpr::Integer(n) => print!("Integer: {},", n),
            _ => (),
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            LispExpr::Integer(n) => Some(format!("{}", n).to_string()),
            LispExpr::Bool(b) => Some(format!("{}", b).to_string()),
            LispExpr::Cons(lhs, rhs) => {
                Some(format!("({} {})", lhs.to_string()?, rhs.to_string()?)
                    .to_string())
            }
            _ => None,
        }
    }

    pub fn eval(&self, env: &mut LispEnv) -> ReturnType {
        match self {
            LispExpr::List(list) => {
                if list.is_empty() {
                    return Ok(LispExpr::Null);
                }

                // handle special forms
                match &list[0].extract_symbol()?.as_str() {
                    &"+" => {
                        if list.len() < 2 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let ans = list[1..]
                            .iter()
                            .map(|a| a.eval(env)?.extract_int())
                            .collect::<Result<Vec<_>, _>>()?
                            .iter()
                            .cloned()
                            .sum();
                        return Ok(LispExpr::Integer(ans));
                    }

                    &"*" => {
                        if list.len() < 2 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let ans = list[1..]
                            .iter()
                            .map(|a| a.eval(env)?.extract_int())
                            .collect::<Result<Vec<_>, _>>()?
                            .iter()
                            .cloned()
                            .product();
                        return Ok(LispExpr::Integer(ans));
                    }

                    &"-" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let first = list[1].eval(env)?.extract_int()?;
                        let second = list[2].eval(env)?.extract_int()?;
                        let ans = first - second;
                        return Ok(LispExpr::Integer(ans));
                    }

                    &"eq?" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?;
                        let rhs = list[2].eval(env)?;
                        let ans = match (lhs, rhs) {
                            (LispExpr::Integer(l), LispExpr::Integer(r)) => l == r,
                            (LispExpr::Symbol(l), LispExpr::Symbol(r)) => l == r,
                            (LispExpr::Bool(l), LispExpr::Bool(r)) => l == r,
                            _ => false,
                        };
                        return Ok(LispExpr::Bool(ans));
                    }

                    &"<" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?.extract_int()?;
                        let rhs = list[2].eval(env)?.extract_int()?;
                        return Ok(LispExpr::Bool(lhs < rhs));
                    }

                    &">" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?.extract_int()?;
                        let rhs = list[2].eval(env)?.extract_int()?;
                        return Ok(LispExpr::Bool(lhs > rhs));
                    }

                    &"<=" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?.extract_int()?;
                        let rhs = list[2].eval(env)?.extract_int()?;
                        return Ok(LispExpr::Bool(lhs <= rhs));
                    }

                    &">=" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?.extract_int()?;
                        let rhs = list[2].eval(env)?.extract_int()?;
                        return Ok(LispExpr::Bool(lhs >= rhs));
                    }

                    &"cons" => {
                        if list.len() != 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let lhs = list[1].eval(env)?;
                        let rhs = list[2].eval(env)?;
                        return Ok(LispExpr::Cons(Box::new(lhs), Box::new(rhs)));
                    }

                    &"car" => {
                        if list.len() != 2 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let res = list[1].eval(env)?;
                        if let LispExpr::Cons(lhs, _) = res {
                            return Ok(*lhs);
                        } else {
                            return Err(LispErr::TypeError("expected cons".to_string()));
                        }
                    }

                    &"cdr" => {
                        if list.len() != 2 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let res = list[1].eval(env)?;
                        if let LispExpr::Cons(_, rhs) = res {
                            return Ok(*rhs);
                        } else {
                            return Err(LispErr::TypeError("expected cons".to_string()));
                        }
                    }


                    &"define" => {
                        if list.len() < 3 {
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
                                let params = lst[1..].to_vec();
                                let body = list[2..].to_vec();

                                env.insert(
                                    fname,
                                    LispExpr::Func(Box::new(LispFunc { params, body })),
                                );

                                return Ok(LispExpr::Null);
                            }
                            _ => {
                                return Err(LispErr::TypeError("invalid syntax".to_string()));
                            }
                        }
                    }

                    &"if" => {
                        if list.len() != 4 {
                            return Err(LispErr::ArityMismatch);
                        }
                        if list[1].eval(env)?.extract_bool()? {
                            return list[2].eval(env);
                        } else {
                            return list[3].eval(env);
                        }
                    }

                    &"cond" => {
                        if list.len() < 3 {
                            return Err(LispErr::ArityMismatch);
                        }
                        for i in 1..list.len()-1 {
                            let c = list[i].extract_list()?;
                            if c.len() != 2 {
                                return Err(LispErr::ArityMismatch);
                            }
                            if c[0].eval(env)?.extract_bool()? {
                                return c[1].eval(env);
                            }
                        }
                        let c = list.last()
                            .expect("list empty")
                            .extract_list()?;
                        if c.len() != 2 {
                            return Err(LispErr::ArityMismatch);
                        }
                        let b = match c[0].clone() {
                            LispExpr::Bool(b) => Ok(b),
                            LispExpr::List(_) => c[0].eval(env)?.extract_bool(),
                            LispExpr::Symbol(s) => if s == "else".to_string() {
                                Ok(true)
                            } else {
                                Err(LispErr::SyntaxError("expected `else`".to_string()))
                            },
                            _ => Err(LispErr::TypeError("expected bool or `else`".to_string())),
                        }?;

                        if b {
                            return c[1].eval(env);
                        } else {
                            return Ok(LispExpr::Null);
                        }
                    }

                    _ => (),
                }

                let func = &list[0].eval(env)?.extract_fn()?;
                let args = &list[1..];

                if args.len() != func.params.len() {
                    return Err(LispErr::ArityMismatch);
                }

                // recursively evaluate until the return value is not an atom
                func.call(args.to_vec(), env)
            }
            LispExpr::Symbol(s) => match env.get(s) {
                Some(e) => e.eval(env),
                None => Err(LispErr::NameError),
            },
            _ => Ok(self.clone()),
        }
    }
}

#[derive(Debug)]
pub enum LispErr {
    ArityMismatch,
    NameError,
    TypeError(String),
    SyntaxError(String),
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
                body: vec![LispExpr::Symbol(a)],
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
