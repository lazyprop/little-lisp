    pub fn default() -> LispEnv<'static> {
        let mut env = LispEnv::new();

        use LispExpr::*;

        // addition
        env.insert(
            "+".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    // TODO too many copies?
                    // TODO find a better way to do this
                    let ans = args
                        .iter()
                        .map(|a| a.extract_int())
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(Integer(ans))
                },
                inf_args: true,
                arity: 0,
                argnames: vec![],
            }),
        );

        // subtraction
        env.insert(
            "-".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    let first = args[0].extract_int()?;
                    let ans: i64 = args[1..]
                        .iter()
                        .map(|a| a.extract_int())
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .sum();
                    Ok(Integer(first - ans))
                },
                inf_args: true,
                arity: 0,
                argnames: vec![],
            }),
        );

        // multiplication
        env.insert(
            "*".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    let ans = args
                        .iter()
                        .map(|a| a.extract_int())
                        .collect::<Result<Vec<_>, _>>()?
                        .iter()
                        .cloned()
                        .product();
                    Ok(Integer(ans))
                },
                inf_args: true,
                arity: 0,
                argnames: vec![],
            }),
        );

        env.insert(
            "square".to_string(),
            Func(LispFunc {
                func: |args: ArgsType| -> ReturnType {
                    // TODO remove these clones
                    // TODO should this be wrapped in an Ok?
                    Ok(List(vec![
                        Symbol("*".to_string()),
                        args[0].clone(),
                        args[0].clone(),
                    ]))
                },
                inf_args: false,
                arity: 1,
                argnames: vec![],
            }),
        );

        env.insert(
            // always fails
            "bad-func".to_string(),
            Func(LispFunc {
                func: |_| -> ReturnType { Ok(List(vec![Symbol("not-defined".to_string())])) },
                inf_args: false,
                arity: 0,
                argnames: vec![],
            }),
        );

        env
    }

