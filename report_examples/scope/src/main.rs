// trait Animal {
//     fn speak();
// }

// struct Dog;

// struct Cat;

// impl Animal for Dog {
//     fn speak() {
//         println!("Woof");
//     }
// }

// impl Animal for Cat {
//     fn speak() {
//         println!("Meow");
//     }
// }

// fn test_animals() {
//     let my_pet: &dyn Animal;
// }

fn main() {
    {
        let var2 = 20;

        println!("{}", var2);
    }

    println!("{}", var2);
}
