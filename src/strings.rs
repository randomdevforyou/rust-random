fn main(){

    //declare string 

    let hello = String::from("Hello"); //immutable string

    //hello.push_str( "World"); // this is not possible as hello is immutable

    let mut hello = String::from("Hello"); //mutable string

    hello.push_str( "World"); // this is possible as hello is mutable

    let hello = String::from("Hello"); //immutable string

    let lowercase = hello.to_lowercase();

    let uppercase = hello.to_uppercase();

    println!("lowercase {} of {}", lowercase, hello);
    
    println!("uppercase {} of {}", uppercase, hello);

    //slices

    let hello = String::from("Hello, World");

    let hello_slice = &hello[0..5];

    println!("slice of {} is {}", hello, hello_slice);

    //make mutable slice

    let mut hello = String::from("Hello, World");

    let hello_slice = &mut hello[0..5];

    let word_slice = &mut hello[7..];


    //change one character on the string

    let hello = String::from("Hello, World");

    let new_hello = hello.replace("Hello", "Hi");

    println!("new string is {}", new_hello);

    //count length of string

    let hello = String::from("Hello, World");

    let length = hello.len();

    println!("length of {} is {}", hello, length);

    // reverse string

    let hello = String::from("Hello, World");

    let reverse = hello.chars().rev().collect::<String>();

    


    // println!("First slice of {} is {}", hello, hello_slice);

    // println!("Second slice of {} is {}", hello, word_slice);



    








    //define string slice
    let hello = "Hello";
    let world = ", World";

    //concat 2 string slices
    let hello_world = hello.to_string() + world;

    println!(" slice contenation {}", hello_world);

    //concatenate using 

    let hello_string = String::from("Hello");

    let wold_string = String::from(", World");

    let concatenated = hello_string.clone() + &wold_string; // 

    println!(" string contenation using + operator {} and hello_string {} is intact", concatenated, hello_string);

    //concatenation using push_str

    let mut hello_string = String::from("Hello");

    let wold_string = String::from(", World");

    hello_string.push_str(&wold_string);

    println!(" string contenation using push_str {} ", hello_string);
    

    // concatenate chars

    let mut hello_string = String::from("Hell");

    hello_string.push('o');
    hello_string.push_str("o, World");

    println!(" string contenation using push_str {} ", hello_string);

    //concatenate using format macro

    let hello_string = String::from("Hello");

    let world_string = String::from(", World");

    let hello_world = format!("{}{}", hello_string, world_string);

    println!(" string contenation using format macro {}, {} ", hello_world, hello_string);

}