struct User{
    pwd: u32,
    //error: email: $str
    email: String,
    active: bool,
}
struct UserT(u32, String, bool);


#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
fn main() {

    let user1 = User{
        pwd: 000000000,
        email: String::from("abc@qq.com"),
        active: true,
    };
    let user2 = build_user(11111111,String::from("123@qq.com"));
    let user3 = UserT(10101010, String::from("111@qq.com"),true);
    let rec1 = Rectangle{
        width: 10,
        height: 10,
    };
    //{:#?} make rec1 output in a readable format
    println!("{:#?}", rec1);
}

fn build_user(pwd: u32, email: String) -> User{
    let user = User{
        //same: pwd: pwd
        pwd,
        //same: email:email
        email,
        active: true,
    };
    user
}


