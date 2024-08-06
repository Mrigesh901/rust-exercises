fn main() {
    let is_even: bool = false;
    // let is_even: bool = true;
    check_even_odd(is_even);
}

fn check_even_odd (is_even: bool) {
    if is_even {
        println!("The number is even");
    }
    else {
        println!("The number is not even");
    }
}