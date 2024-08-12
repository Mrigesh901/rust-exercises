struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let name: String = String::from("Alice");
    let user:User = User {
        name: name,age: 30,
        active: true,
    };

    println!("{} is {} years old and activity value is {}", user.name, user.age, user.active);
}


