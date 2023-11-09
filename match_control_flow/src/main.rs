
fn main() {
    let mut dice_roll = 8;
    match dice_roll {
        0 => stop(),
        9 => win(),
        //other meets the requirement that match flow must contain all possibility.
        other => pace(dice_roll),
    }
    let mut dice_roll = Some(9);
    if let Some(9) = dice_roll{
        win();
    }
    else{
        pace(dice_roll);
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

