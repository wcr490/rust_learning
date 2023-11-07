#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle {
    //it's a custom to build a method named "new"
    //rust provide us with a shorthand to implement it.
    //same:
    // fn new(width: u32, height: u32) -> Rectangle{
    //     let rec = Rectangle{
    //         width: width,
    //         height: height,
    //     };
    //     return rec;
    // }
    fn new(width: u32, height: u32) -> Self{
        Self{
            width: width,
            height: height,
        }
    }
    fn width(&self) -> bool{
        self.width > 0
    }
    fn can_hold(&self, comp_obj: &Rectangle) -> bool{
        self.width > comp_obj.width && self.height > comp_obj.height
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 10,
        height: 20
    };
    let rec2 = Rectangle{
        width: 30,
        height: 40
    };
    //prone error: rec1.width
    if rec1.width(){
        println!("width is fine");
    }
    println!("{}", rec1.can_hold(&rec2));
    println!("{}", rec2.can_hold(&rec1));

}
