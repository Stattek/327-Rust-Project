fn main() {
    // these are our primitive datatypes
    integers_example();
    floating_point_example();
    bool_example();
    char_example();

    // here are some more advanced datatypes
    tuple_example();
    array_example();
}

fn integers_example() {
    // here is an example of explicitly specifying a type
    // with a type annotation
    // i32 is the default value for integers
    let num1: i32 = 2;

    // these integer types specify the number of bits used to hold the integer
    // also, there are signed and unsigned versions of all of them
    // and obviously, signed and unsigned versions of integers can store different
    // amounts of data because the first bit in a signed integer is not for the sign
    // and not numerical data
    let num2: i8 = 8;
    let num3: i16 = 16;
    let num4: i32 = 32;
    let num5: i64 = 64;
    let num6: i128 = 128;
    let num7: u128 = 128;

    println!(
        "{} {} {} {} {} {} {}\n",
        num1, num2, num3, num4, num5, num6, num7
    );
}

fn floating_point_example() {
    // we have two different types of floats
    // f32 is a "float" while an f64 is a "double"
    // The default is f64
    let floating_point1: f32 = 10.4;

    let floating_point2: f64 = 60.924;

    println!("{} {}\n", floating_point1, floating_point2);
}

fn bool_example() {
    // examples of boolean values
    let my_bool: bool = true;
    let my_bool2 = false;

    println!("{} {}\n", my_bool, my_bool2);
}

fn char_example() {
    // we have a char datatype like in C++ and SML
    // this should be familiar
    let letter: char = 'e';
    let letter2 = '\n';
    let letter3 = 't';

    println!("{} {} {} \n", letter, letter2, letter3);
}

fn tuple_example() {
    // tuples, like in SML, have a type that is made up of
    // every element's type
    let my_tuple = (1, true, 's');

    // if I were to rewrite my_tuple explicitly, it would look like this:
    let my_tuple: (i32, bool, char) = (1, true, 's');

    // println!("{}\n", my_tuple); //this will give an error because println can't format it

    //how we access an element of the tuple
    print!("{}", my_tuple.0);
    print!(" {}", my_tuple.1);
    print!(" {}\n", my_tuple.2);

    //how to change an element of a mutable tuple
    let mut my_tuple = my_tuple;

    print!("{}", my_tuple.0);
    print!(" {}", my_tuple.1);
    print!(" {}\n", my_tuple.2);

    my_tuple.0 = 15;
    my_tuple.1 = false;
    my_tuple.2 = 'e';

    print!("{}", my_tuple.0);
    print!(" {}", my_tuple.1);
    print!(" {}\n", my_tuple.2);

    //or we could change it like this
    my_tuple = (12, true, 'f');
    println!("{} {} {}\n", my_tuple.0, my_tuple.1, my_tuple.2);
}

fn array_example() {
    // arrays are created very similarly to C++
    // and are indexed very similarly as well
    let my_array = [1, 2, 3, 4, 5];

    println!(
        "{} {} {} {} {}",
        my_array[0], my_array[1], my_array[2], my_array[3], my_array[4]
    );

    // my_array[0] = 3; // this gives an error because the array is not mutable
    // to be able to change values in an array, it must be mutable
    let mut my_array2 = my_array;

    my_array2[0] = 21;
    println!(
        "{} {} {} {} {}",
        my_array2[1], my_array2[1], my_array2[2], my_array2[3], my_array2[4]
    );

    // Now what if we make an array but don't initialize anything?
    let mut my_array3: [i32; 2];

    // println!("{} {}", my_array3[1], my_array3[1]); // this will give an error
    // this is because Rust does not initialize arrays for you, like C++
    // you have to initialize the variables yourself

    // This will also give you an error. This is just not allowed in Rust
    // because it doesn't know what type the array is and it doesn't know
    // how many elements it should have
    // let my_array4 = [];

    // even this isn't allowed
    // let my_array5:[i32,2] = [];
}
