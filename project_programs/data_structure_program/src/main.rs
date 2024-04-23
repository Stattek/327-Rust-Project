use std::{str::FromStr, vec};

struct VideoGame {
    name: String,
    genre: String,
}

struct Library {
    // keep a list of games
}



fn main() {
    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push("hello".to_string());
    my_vec.push("world".to_string());
    my_vec.push("this".to_string());
    my_vec.push("is".to_string());
    my_vec.push("a".to_string());
    my_vec.push("test".to_string());

    for string in my_vec {
        println!("{}", string)
    }
}
