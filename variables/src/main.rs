fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const TREE_PLUS_SIX: u32 = 3 + 6;

    let array = [0;12];
    let tuple: (u8, u16, u32, u64, u128) = (1, 2, 3, 4, 5);
    let (a, b, c, d, e) = tuple;
    let f = tuple.0;
    let g = tuple.1;
    let h = array[0];
    let i = array[1];

    //distinguish statement and expression
    let j: u32 = 1;//statement, which doesn't have a return value.
    //specially     let j = k = 1   Writing in this way will cause errors in Rust, which is allowed in C
    //From this point of view, In Rust, there's a hardcore difference between statements and expressions
    //there's a authentic expression. see below.
    let k: u32 = {
        x = 100;
        j
    };
    println!("{k}");
    println!("{x}");

    //bool is available in Rust and is separate to int,which is usually used in control flow
    if j != 1{
        println!("j changes");
    }
    else {
        println!("j = 1");
    }

    if j < 100{
        println!("j < 100");
    }
    else if j >500{
        println!("j > 500");
    }
    else{
        println!("100<=j<=500");
    }

    let mut condition = false;
    let l: u32 = if condition{1}else{2};
    //the same : let l = if condition{1}else{2};

    let mut counter = 0;
    let m = loop{
        counter += 1;
        if counter == 10{
            break (counter*10);
        }
    };

    'out_loop: loop{
        counter -= 1;
        loop{
            counter += 2;
            if counter > 15{
                break 'out_loop;
            }
        }
    }

    while !condition{
        condition = true;
    }

    let a = [1, 3, 5];
    for element in a(){
        println!("{element}");
    }

    for number in (1..4).rev(){
        println!("{number}");
    }
}

fn other_function(value: u32, input: char){
    //the position probably is not such important in rust comparing to C
}

fn double(value: u32) -> u32{
    value*2
}