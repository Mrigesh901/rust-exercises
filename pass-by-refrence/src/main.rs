// A reference is created by & and dereferenced by *.

fn main() {
    let x = 4;
    let ref1 = by_ref(&x);
    let ref2 = by_ref(&20);
    println!("{}, {}", ref1, ref2);

    // for mutable values
    
    let mut mut_ref = 4.0;
    println!("Original float is {}", mut_ref);
    by_ref_mutate(&mut mut_ref);
    println!("Modified float is : {}", mut_ref);
}

fn by_ref(a: &i32) ->i32 {
    *a+5
}

// if you want to change the value at that refrence for example x = 4.0 to x = 4.5, use mutable

fn by_ref_mutate(a: &mut f32) {
    *a = 4.5;

}