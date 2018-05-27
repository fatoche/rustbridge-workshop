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
- numbers can be written as `let num = 375u32` -> append the type to the number -> not really common, weird aesthetics
- Strings "some string" vs. Char 'c'
- size: different lengths, depending on the platform
- just use the compiler to tell you which types to write in function/variable declarations! (yay, it's not yelling at us, it's helping :D )
- containers
	- array: fixed size, let color = [255, 0, 255], color[2]
		- []-indexing panics with invalid indices
		- .get() indexing returns None (Option) at runtime
	- Vec<T>: variable size
	- Tuples
		- syntax sugar: tuple[2] vs tuple.2
	- Strings
		- to_owned, into return String
		- &str is a reference, there can be only one mutable reference at any time
		- String is owned

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
- lambdas: |x| x % 2 == 0

Flow control
-----------
- match: exhaustive -> use _ for remaining cases
- for i in blubb {}
	- exclusive range: 0..100
	- inclusive range: 0..=100

Tests
-----
- use #[test] before the test function
- ignore test: #[ignore]

Error Handling
--------------
- Option
	- Some(value) or None
	- Option.unwrap() -> panics if option is None
	- option.expect(my_message) -> panics with message my_message if option is None, nice if you are still learning to have a sensible error message
	- option.unwrap_or_else() -> confusing way to avoid matching on option
- Result
	- Ok(value) or Err(error)
	- use Result instead of exception handling
- ? operator: lets errors bubble up
	- can be used on an Option/Result in any function that also returns an Option/Result

Strings
-------
- &str = string slice
	- Referenz
- String
	- growable
	- allocated ?? (Steve: see chapter 4 in the Book)
	- always owned? ("".to_owned() and "".to_string() have a different history, but now do exactly the same: copy the data from a &str, make a String out of it and take ownership)

Ownership
--------
- see slides and intorust.com

Imports
------
1. add dependency in Cargo.toml
2. extern crate crate_name;
3. use crate_name::Struct [as my_struct_name]; OR use crate_name::{Struct1, Struct2};


Issues
-----
- no error if you forget ; after a println!, you only get the error if you add other statements (not println, not Struct definition) after it



Website
=======

Crates
------
- simple_server: easy, good for learning but not for production
	- must of the complicated/internal stuff is hidden
- rocket: not much boilerplate
- motivations -> there are open issues!
	- collection of motivational messages
- pick_one -> picks a random element from an array
- python: flask -> minimal, easy web framework

Server
-----
- new(callback)
- callback is a closure that takes a request (stuff that we listen to) and an empty response that the server will fill
- request has a method (GET or POST) and a URI
