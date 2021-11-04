# Little Lisp Interpreter in Rust

Right now it's just a poorly written calculator.


The `main.py` is what I was originally trying (huge failure). Turns out it's
much easier to do in Rust with all the types.

## TODO
- [ ] REFACTOR
  - [ ] Think about why all the `clones` and think of something better.
  - [ ] Get rid of all the unnecessary `match`es.
  - [ ] Get rid of all the unnecessary copies.
  - [ ] Be careful where to borrow and where to take ownership.
  - [ ] Make everything functional.
- [ ] Better errror handling.
- [ ] More tests.
- [ ] Write a parser.
- [ ] Write a REPL.
- [ ] Add more things to default environment.
