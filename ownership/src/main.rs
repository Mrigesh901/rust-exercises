fn main() {
    let first = String::from("A");
    let second = first;

    // println!("{}",first);
    //  let first = String::from("A");
//   |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
//   3 |     let second = first;
//     |                  ----- value moved here
//   4 |
//   5 |     println!("{}",first);
//     |                   ^^^^^ value borrowed here after move
//     |
//     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
//   help: consider cloning the value if the performance cost is acceptable
//     |
//   3 |     let second = first.clone();
//     |                       ++++++++
// if we clone, it is very expensive for heap memory, 2 copies will be created

    println!("{}",second);

    // ///////////////////////////////////////////////////////////////
// another way

    let initial_str = String::from("I am initial");
    take_ownership(initial_str);



////////////////////////////////////////////////////////////////////////   

//     println!("{}",initial_str); if you try to print initial_str, it will give error bcz the ownership has moved to another function
//     let initial_str = String::from("I am initial");
//    |         ----------- move occurs because `initial_str` has type `String`, which does not implement the `Copy` trait
// 27 |     take_ownership(initial_str);
//    |                    ----------- value moved here
// 28 |     println!("{}",initial_str);
//    |                   ^^^^^^^^^^^ value borrowed here after move
//    |
// note: consider changing this parameter type in function `take_ownership` to borrow instead if owning the value isn't necessary
//   --> src/main.rs:33:31
//    |
// 33 | fn take_ownership(second_str: String) {
//    |    --------------             ^^^^^^ this parameter takes ownership of the value
//    |    |
//    |    in this function
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 27 |     take_ownership(initial_str.clone());
//    |                               ++++++++
/////////////////////////////////////////////////////////////////////////////////////////////////

}

fn take_ownership(second_str: String) {
    println!("{}",second_str)
}




// another thing that can be done is to pass the ownership in take_ownership function to some_other_str , in this way data will be passed back to main
// fn take_ownership(second_str: String) -> String {
//      return second_str
// }


// In the main function set the ownership to original var or create a new
// third_str = take_ownership(initial_str);
// or
// let mut initial_str  = String::from("I am initial");
// initial_str = take_ownership(initial_str);
