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
    prices[0] = 2; // apples are on sale ...
    let goods = vec!["Apple", "Pear", "Banana"];
    for (good, price) in goods.iter().zip(prices.iter()) {
        println!("A {} costs {} euros.", good, price);
    }

    // structs
    #[derive(Debug)]
    struct Dog {
        name: String,
        age: u8,
    }

    let name = "Fuchur".to_string();
    let age = 7;

    let fuchur = Dog {
        name,
        age,
    };
    let kyango = Dog {
        name: "Kyango".to_string(),
        age: 1,
    };
    println!("Fuchur: {:?}, Kyango: {:?}", fuchur, kyango);
}

fn add_fifty(n: i32) -> i32 {
    if n < 35 {
        n + 50
    } else {
        n + 50
    }
}

#[test]
fn test_add_fifty() {
    assert_eq!(add_fifty(5), 55);
}

#[ignore]
#[test]
fn stupid_test() {
    assert_eq!(add_fifty(5), 45);
}
