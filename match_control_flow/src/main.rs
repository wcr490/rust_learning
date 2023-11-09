
fn main() {
    let mut dice_roll = 8;
    match dice_roll {
        0 => stop(),
        9 => win(),
        other => pace(dice_roll),
    }
}

fn stop(){
    println!("Stop!");
}
fn win(){
    println!("You win");
}

fn pace(mut dice_roll: u32){
    dice_roll = 0;
}
