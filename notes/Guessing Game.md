# Guessing Game

### Updated as of 10/28/2026


## Important learnings
* Println and Print are not functions but rather macros denoted by !
* Variables can optionally declare their type using the : type syntax.
* Variables must be declared as mutable `mut` or not in order for them to be modified
* No postfix increment (kinda dumb)
* `&str` is just a pointer to a string of characters, immutable. Whereas a `String` is an actual mutable string of characters
* `print!()` Specifically requires the stdout to be flushed as it usually waits for a '\n' before moving on
* `loop {}` is an easy way to make a generic infinite loop: `while (true)` essentially
* Case expressions using the `match` keyword
* Error handling seems pretty straight forwards, using the Result type to check if the result is `Ok` or `Err`
* Without custom handling it seems like most of these functions just panic and crash the program.

## The main thing
* Seems like doing `&varname` is required for some functions, most likely has to do with whatever the borrow checker is.
* I'm not sure yet but it seems like if I don't borrow a variable I cannot perform certain actions on it which helps with safety somehow.

