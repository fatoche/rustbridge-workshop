Cargo
-----
- cargo new --bin/--lib
	- default: bin
	- Cargo.toml, 
	- lib: lib.rs, ignore Cargo.lock -> warum??
	- bin: main.rs, Cargo.lock nicht ignoriert

Errors
-----
- bad/inscrutable error message:
	- report an issue at rust-lang/rust
	- special tag for those issues
	- it may take some time for someone to react (there are a LOT of open compiler issues!), but they are appreciated and will be processed eventually

Types
-----
- integers: default i32
- floating-point numbers: default f64
- if the compiler cannot determinate the exact type, it errors (only exception: numbers)
- Strings "some string" vs. Char 'c'
- size: different lengths, depending on the platform
- just use the compiler to tell you which types to write in function/variable declarations! (yay, it's not yelling at us, it's helping :D )

Printing
-------
- macro println!()
- first argument: String with {} for arguments, e.g. "Hallo, {}"
- other arguments: arguments in the order of their curlys -> println!("Hallo, {} and {}!", name_one, name_two)
- can you use one argument several times? something like println!("first: {0}, second: {1}, first again: {0}", first, second)

Functions
--------
- declarations are possible inside other functions
- the result of the last statement (if it does not end with a semicolon) is automatically returned -> explicit return statement not required (but can be put there)