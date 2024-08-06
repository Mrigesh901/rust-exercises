fn main() {
    // let sqr = sqr(2);   giving int as arg will give compile error because signature of function expects float
    let sqr = sqr(12.0);
    println!("{}", sqr);
}

fn sqr(x: f64) -> f64 {
    // You will actually rarely see functions written using a return statement. More often, it will look like this:
    // This is because the body of the function (inside {}) has the value of its last expression, just like with if-as-an-expression.

    // Since semicolons are inserted semi-automatically by human fingers, you might add it here and get the error:
    // The () type is the empty type, nada, void, zilch, nothing. Everything in Rust has a value, but sometimes it's just nothing.
    x*x
}
