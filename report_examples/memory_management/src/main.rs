fn main() {
    box_test();
}

fn box_test(){

    let my_heap_data = Box::new(24);

    println!("{}", my_heap_data);
    
}