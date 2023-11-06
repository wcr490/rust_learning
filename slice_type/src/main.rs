fn main() {
    let s = String::from("Slide");
    // prone error:     let s_a = s[0..1];
    let s_a = &s[0..1];
    let s_b = &s[0..s.len()];
    //when using referencing, the reference will not be cleared
    //until the main project(owner) is cleared
}

//
fn find_first_word(value: &String) -> &str{     //slice type should be stored by &str
                                                //same as fn find_first_word(value: &Str) -> &str
    let value_byte = value.as_bytes();
    for (i, &item) in value_byte.iter().enumerate(){
        if item == b' '{    //b is a signal for bytes.
            return &value[0..i];
        }
    }
    &value
}

