fn main() {
    
    let is_true: bool = true;
    check_true_false(is_true);



    print_even_odd();


    let x: i8 = 9;
    let ans = check_even(x);
    if ans == true {
        println!("The number is even");
    }
    else {
        println!("The number is odd");
    }
}


/////////////////////////////////////////////


// check if a boolean value is true or false
fn check_true_false (is_true: bool) {
    if is_true {
        println!("The value is {}", is_true);
    }
    else {
        println!("The number is not true");
    }
}

///////////////////////////////////////////

// print all even numbers bw 1..10
fn print_even_odd() {
    for i in 1..10 {
        let even_odd = if i%2 == 0 {"even"} else {"odd"};
        println!("{} is {}",i,even_odd);
    }
}

///////////////////////////////////////////



fn check_even(a: i8) -> bool {
    if a%2 == 0 {
        return true;
    }
    else {
        return false;
    }
}