/* fn main() {
    open_store("Toronto");
    bake_pizza(5, "pepperoni");
    swim_in_profit();
    open_store("Calgary");
    bake_pizza(5, "mushroom");
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas.");
}

fn swim_in_profit() {
    println!("So much $$$, so little time.")
} */

fn main() {
    // println!("{}", square(10));
    // let result = mystery();
    // let multiplier = 3;

    // let calculation = {
    //     let value = 5 + 4;
    //     value * multiplier
    // };

    // println!("{calculation}")
    //
    apply_to_jobs(35, "Rust Developer");

    println!("{}", is_even(8));
    println!("{}", is_even(9));

    println!("{:?}", alphabets("ajhdasd"));
}

// fn mystery() {
//     println!("hello world");
// }

// fn square(number: i32) -> i32 {
//     number * number
// }

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs.")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains('z'), text.contains('a'))
    // let let_a = text.contains('a');
    // let let_z = text.contains('z');
    // (let_a, let_z)
}

/*
Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains

Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/
