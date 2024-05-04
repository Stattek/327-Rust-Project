fn main() {
    println!("Hello, world!");
}

fn type_inference() {
    let my_var = 19;
    let my_str = "Hello, World!";
}

fn print_generic<T>(value: T) {
    println!("{}", value);
}

fn get_string() -> String {
    String::from("This string is from get_string().")
}
