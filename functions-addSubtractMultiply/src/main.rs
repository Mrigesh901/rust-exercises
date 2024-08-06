pub fn add(a: i32, b: i32) -> i32 {
    // Step 1: implement addition
    let sum = a+b;
    return sum;
}

pub fn subtract(a: i32, b: i32) -> i32 {
    let sum = a-b;
    return sum;
}

pub fn multiply(a: i32, b: i32) -> i32 {
    let sum = a*b;
    return sum;
}

fn main() {
    let result = add(2, 3);
    println!("Result is {}", result);
    assert_eq!(result, 5);
    
    let result = subtract(5, 3);
    println!("Result is {}", result);
    assert_eq!(result, 2);
    
    let result = multiply(2, 3);
    println!("Result is {}", result);
    assert_eq!(result, 6);
}