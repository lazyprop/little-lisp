# Little Lisp Interpreter in Rust

At this point this is just a poorly written calculator.


The `main.py` is what I was originally trying (huge failure). Turns out it's
much easier to do in Rust with all the types.

## TODO
- [ ] REFACTOR
  - [ ] Think about why all the `clones` and think of something better.
  - [x] Get rid of all the unnecessary `match`es.
  - [x] Get rid of all the unnecessary copies.
  - [ ] Be careful where to borrow and where to take ownership.
  - [x] Make everything functional.
  - [ ] Find a better way to do built-in `+` and `*` functions
  - [x] Write more `LispExpr.parse()` functions.
  - [ ] Think of a better name for `LispExpr.parse_type()`. It's not actually a
    parser.
- [x] Better error handling.
- [ ] More general function definition. Fixed arity functions. Typechecking
  arguments.
  - [ ] Figure out what type should the function be and return.
        Is it a `LispExpr`? Is it a `LispFunc`. In `+` etc. we're evaluating the
        function there but not while composing functions. This causes type
        mismatch.
- [ ] More tests.
- [ ] Write a parser.
- [ ] Write a REPL.
- [ ] Add more things to default environment.
