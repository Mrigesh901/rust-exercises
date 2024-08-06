fn main() {
    let x: u64 = 5;
    println!("Factorial of {} is {}", x, factorial(x));
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}