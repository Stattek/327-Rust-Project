struct Node {
    // stores its next node on the heap
    // the next node can exist or not exist, which is why it is an Option
    pub next: Option<Box<Node>>,
    pub data: i32,
}

impl Node {
    pub fn new(data: Option<i32>) -> Self {
        Self {
            //gives next a default value of None
            next: None,
            // makes data a box
            data: data.expect("Inserted no value into a node."),
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new(data: Option<i32>) -> Self {
        match data {
            None => Self { head: None },
            Some(_num) => Self {
                head: Some(Box::new(Node::new(data))),
            },
        }
    }
}
