/**
 * This is a doc comment.
 */
fn main() {
    // this is a comment
    println!("Hello, RustBridge Paris!");

    let name: &'static str = "Fuchur";
    let age = 7;
    println!("{} is {} years old.", name, age);
    println!("Someone else is {} years old.", add_fifty(age));

    let color = [255, 0, 255];
    // debug printing (no normal debugging for an array)
    println!("color array: {:?}", color);
    // pretty printing (more helpful for nested structures)
    println!("color: {:#?}", color);

    let index = 9;
    println!("The 10th element is {:?}", color.get(index));
    if color.get(index) == None {
        println!("The index is out of bounds!");
    }
    //println!("The 10th element is {:?}", color[index]);

    // Vectors!!!
    let mut prices = vec![3, 4, 5];
    let goods = vec!["Apple", "Pear", "Banana"];
    for (good, price) in goods.iter().zip(prices.iter()) {
        println!("A {} costs {} euros.", good, price)
    }
}

fn add_fifty(n: i32) -> i32 {
    if n < 35 {
        n + 50
    } else {
        n + 50
    }
}
