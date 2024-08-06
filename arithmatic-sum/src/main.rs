// 'Immutable'? A variable that cannot vary? let variables by default can only be assigned a value when declared. 
// Adding the magic word mut (please make this variable mutable) does the trick:

//  sum +=i;
//   |         ^^^^^^^ cannot assign twice to immutable variable
// if we want to change values of variables in runtime, we need to make them mutable using mut keyword


fn main() {
    arithmatic_sum();
}

fn arithmatic_sum() {
    let mut sum = 0;
    for i in 1..101 {
        sum +=i;
    }
    println!("Sum from 1 to 100 is : {}", sum);
}