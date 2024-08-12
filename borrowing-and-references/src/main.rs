fn main() {
    let my_string = String::from("Hello");
    borrow_ownership(&my_string);
    println!("{}",my_string);

}

// fn main() {
//     let mut s1 = String::from("Hi");
//     let s2: &mut String = &mut s1;
//     s2.push_str(" World");
//     println!("{}",s2);

//     let s3 = s1;
//     // let s3: &mut String = & mut s1;
//     println!("{}",s3)


// }


fn borrow_ownership(some_string: &String) {
    println!("{}", some_string);
}
