struct User{
    pwd: u32,
    //error: email: $str
    email: String,
    active: bool,
}
struct UserT(u32, String, bool);
fn main() {

    let user1 = User{
        pwd: 000000000,
        email: String::from("abc@qq.com"),
        active: true,
    };
    let user2 = build_user(11111111,String::from("123@qq.com"));
    let user3 = UserT(10101010, String::from("111@qq.com"),true);
}

fn build_user(pwd: u32, email: String) -> User{
    let user = User{
        //same: pwd: pwd
        pwd,
        //same: email:email
        email:emai,
        active: true,
    };
    user
}
