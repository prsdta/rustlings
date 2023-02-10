// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // Compiler can infer the type based on usage?

    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("mango"), 2);
    basket.insert(String::from("papaya"), 2);

    // Shorthand alternative
    // let mut basket = HashMap::from([
    //     (String::from("banana"), 2),
    //     (String::from("mango"), 2),
    //     (String::from("papaya"), 2),
    // ]);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
