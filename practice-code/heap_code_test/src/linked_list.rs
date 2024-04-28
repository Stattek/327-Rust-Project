pub struct Node {
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

    pub fn push_back(&mut self, data: i32) {
        // get the current node in the list, starting at the head

        //get a mutable reference to the head
        let mut cur_node = self.head.as_mut();

        // find the node we want to add
        for _i in 0..self.size - 1 {
            // increment the head
            if let Some(next_node) = cur_node {
                cur_node = next_node.next.as_mut();
            }
        }

        // add the element to the end of the list
        if let Some(node) = cur_node {
            node.next = Some(Box::new(Node::new(data)));
        }

        self.size += 1;
    }

    pub fn at(&mut self, index: &i32) -> Option<&mut Box<Node>> {
        if *index < self.size {
            //get a mutable reference to the head
            let mut cur_node = self.head.as_mut();

            // find the node we want to add
            for _i in 0..*index {
                // increment the head
                if let Some(next_node) = cur_node {
                    cur_node = next_node.next.as_mut();
                }
            }

            return cur_node;
        } else {
            return None;
        }
    }

    pub fn print_list(&self) {
        let mut cur_node = self.head.as_ref();

        // print opening bracket
        print!("[");
        // go through the linked list
        for num in 0..self.size {
            // if we have some value, then print it
            if let Some(next_node) = &cur_node {
                print!("{}", next_node.data);

                if num + 1 != self.size {
                    // if this is not the last element, then print a comma
                    print!(", ");
                }
                cur_node = next_node.next.as_ref();
            }
        }

        // print closing bracket
        println!("]");
    }
}
