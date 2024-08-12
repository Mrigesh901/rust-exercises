struct Rect{
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width*self.height
    }
}


fn main() {
    let rect = Rect {
        width:20,
        height:10,
    };
    println!("Hello, world, the area of Rect is {}", rect.area());
}
