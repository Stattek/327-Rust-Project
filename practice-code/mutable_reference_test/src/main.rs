struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut first = Some(Box::new(12));
    let second = Some(Box::new(38));
    let third = Some(Box::new(32));

    let my_ref = &mut first;
    my_ref = 
}
