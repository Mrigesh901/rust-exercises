// Closures in Rust are anonymous functions that can capture variables from their environment. They are similar to functions but have some unique properties that make them powerful and flexible. Closures are often used for short, simple operations and can be defined in a very concise way.

// Understanding Closures
// A closure is defined using a pair of vertical bars | that enclose the parameters, followed by the closure body. Here's a simple example of a closure that adds two numbers:

// let add = |a: i32, b: i32| a + b;
// In this example, add is a closure that takes two parameters, a and b, and returns their sum. You can call this closure just like a function:

// let result = add(2, 3); // result is 5
// Closures can capture variables from their enclosing scope. For example:

// let x = 2;
// let add_x = |a: i32| a + x;
// let result = add_x(3); // result is 5
// In this case, the closure add_x captures the variable x from the surrounding scope and adds it to its parameter a.

pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        // Step 1: Implement here
        a+b
    };

    // Step 2:
    // Create the `subtract_closure` closure that subtracts two `i32` values.
    let subtract_closure = |a, b| {
        // Step 1: Implement here
        a-b
    };

    // Step 3:
    // Create the `multiply_closure` closure that multiplies two `i32` values.
    let multiply_closure = |a, b| {
        // Step 1: Implement here
        a*b
    };

    (add_closure, subtract_closure, multiply_closure)
}

fn main() {
    let (add, subtract, multiply) = create_closures();   
    // a tuple is created and we call the value from that tuple
    let add = add(3,4);
    let subtract = subtract(6,4);
    let multiply = multiply(3,5);
    println!("{}, {}, {}", add, subtract, multiply);
}