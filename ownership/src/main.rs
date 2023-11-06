fn main() {
    //stack and heap is important to our rustaceans
    let mut a = String::from("hello");
    a += ",world";
    a.push_str("!!!");
    {
        let mut b = String::from("ok");
        //error: take_ownership(b);
        //instead
        take_ownership(b.clone());
        println!("{}",b);

        //reference can be a great way to convey variables but not take its ownership.
        referencing_test(&b);
        println!("{}",b);

        mutable_referencing_test(&mut b);
        println!("{}",b);
        //mutable reference must be only

        let c: i32 = 1;
        not_take_ownership(c);
        //that will be fine because of the fixed memory that i32 occupies.
        println!("{}",c);
    }
}

fn take_ownership(value: String){
    println!("{}",value);
}

fn not_take_ownership(value: i32){
    println!("{}",value);
}

fn referencing_test(value: &String)->usize{
    value.len()
    //error: value += "anything"
    //you just borrow it but never own it, implying that you couldn't modify it by default
}

fn mutable_referencing_test(value: &mut String){
    *value += "anything";
}