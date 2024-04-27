struct Node {
    // stores its next node on the heap
    // the next node can exist or not exist, which is why it is an Option
    pub next: Option<Box<Node>>,
    pub data: i32,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self {
            //gives next a default value of None
            next: None,
            // makes data a box
            data: data,
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl LinkedList {
    pub fn new(data: Option<i32>) -> Self {
        // create a linked list with one node
        match data {
            Some(data) => Self {
                head: Some(Box::new(Node::new(data))),
                size: 1,
            },
            None => Self {
                head: None,
                size: 0,
            },
        }
    }

    pub fn add_element(&mut self, data: i32, index: &i32) {
        if *index < self.size {
            // get the current node in the list, starting at the head

            //get a mutable reference to the head
            let mut cur_node = &mut self.head;

            // find the node we want to add
            for i in 0..*index {
                // increment the head
                let next_node = match cur_node {
                    Some(cur_node) => &mut cur_node.next,
                    _ => panic!("Error in finding element"),
                };

                cur_node = next_node;
            }
        }
    }
}
