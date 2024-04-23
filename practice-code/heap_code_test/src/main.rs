// use the module linked_list, which has our code for the linked list
mod linked_list;
// use the linked list from our module
use crate::linked_list::*;

fn main() {
    // let my_node = Node::new(Some(23));

    let my_list = LinkedList::new(Some(32));
    // println!("{}", my_node.data);
    
}
