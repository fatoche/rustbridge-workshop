/**
 * This is a doc comment.
 */
fn main() {
    // this is a comment
    println!("Hello, RustBridge Paris!");

    let name: &'static str = "Fuchur";
    let age = 7;
    println!("{} is {} years old.", name, age);
    println!("Someone else is {} years old.", add_fifty(age))
}

fn add_fifty(n: i32) -> i32 {
    if (n < 35) {
        n + 50
    } else {
        n + 50
    }
}
