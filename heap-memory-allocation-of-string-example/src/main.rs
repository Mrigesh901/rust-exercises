fn main() {
    update_string();
}

fn update_string() {
    //  Strings will sit on heap and they will be referenced from stack
    let mut s: String = String::from("Initial string");
    println!("Before update {} ",s);
    println!("Capacity: {}, Length: {}, pointer: {:p}",s.capacity(),s.len(),s.as_ptr());

    // modify the string, append to it, change the size
    // lets do it multiple times to check does the pointer changes when the size grows, if the pointer will change that means there is no contigous space left to assign

    for _ in 0..100 {
        s.push_str("And some additional text");
        println!("Capacity: {}, Length: {}, pointer: {:p}",s.capacity(),s.len(),s.as_ptr());
    }
}

// we can see that capacity and length also changes as the value of string changes at runtime. The heap asks for a bigger memory allocation each time
// because of this storing data in heap is slow but dynamic ex, vectors,strings
// storing data on stack memory is fast and static i.e numbers,bools