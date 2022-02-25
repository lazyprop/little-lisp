A little Lisp interpreter written in Rust. Implements a small subset of Scheme.
Extremely slow, but works.

Small example:
![Small example](example-screenshot.png)

### To-do's for the next interpreter I write
- Because of the way built-in functions like arithmetic are represented in the
  codebase, they cannot be treated as normal lisp procedures. Trying to pass
  `+` into a `fold`, for example, will result in a name error.
- Too many clones and other hacks to get around Rust's ownership rules. This
  probably does significantly impact the execution speed of the compiler.
- Better, context-aware errors, errors at the parser level for catching syntax
  errors etc.
- Quotes.
- Tail call optimization.
