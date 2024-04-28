// use the module linked_list, which has our code for the linked list
mod linked_list;
// use the linked list from our module
use crate::linked_list::LinkedList;

fn main() {
    // let my_node = Node::new(Some(23));

    let mut my_list = LinkedList::new(Some(32));
    my_list.push_back(42);
    // println!("{}", my_node.data);
    my_list.print_list();
    let my_node = my_list.at(&1);

    if let Some(node) = my_node {
        println!("{}", node.data);
    }
}
