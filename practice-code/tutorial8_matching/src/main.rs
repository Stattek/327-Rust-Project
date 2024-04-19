fn main() {
    for i in 1..=20 {
        matching(i);
    }
}

fn matching(num: i32) {
    print!("This number is ");
    // // now we match the number
    match num {
        1 => println!("a one!"),
        2 => println!("a two."),
        3 => println!("a three."),
        4..=10 => println!("above 3 but at most 10."),
        13..=19 => println!("a teen."),
        _ => println!("anything else."),
    }
}
