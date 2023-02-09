// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

fn foo<'a>() {
    // Any lifetime would work here, as long as it's the same as the function's lifetime
    let mut shopping_list: Vec<&'a str> = Vec::new();
    shopping_list.push("milk");
}

fn main() {
    foo()
}
