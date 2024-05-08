fn main() {
    // let mut val = 4;
    // let mut val2 = &mut val;
    // let val3 = &val;

    // println!("{}", val2);
    // println!("{}", val);

    // val = 3;
    // println!("{}", val2);
    // println!("{}", val);

    // *val2 = 2;

    // println!("{}", val2);
    // println!("{}", val);

    // box_test();

    let my_val = 19;
    fun(&my_val, my_val);
}

fn dangling_pointer() {
    let reference;
    {
        let value = 19;
        reference = &value;
    }

    println!("{}", reference);
}

fn fun(val: &i32, val2: i32) {
    println!("{}", val);
    println!("{}", val2);
}

fn box_test() {
    let my_heap_data = Box::new(24);

    println!("{}", my_heap_data);
}

fn move_test() {
    let my_value = 19;

    // my_value's value of 19 is moved to second_value and now second_value owns 19
    let second_value = my_value;
}
