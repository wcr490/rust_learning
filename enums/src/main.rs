enum IpAddrType{
    V4Num(u8, u8, u8, u8),
    V4STRING(String),
    V6STRING(String),
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let ip_v4 = IpAddrType::V4Num(127,0,0,1);
    let ip_v4_string = IpAddrType::V4STRING(String::from("127.0.0.1"));
    let ip_v6 = IpAddrType::V6STRING(String::from(""));

    //option<T> is an important design in Rust to avoid none errors
    //learn more later
    let a = Some(5);
    let absent: Option<u32> = None;
}
