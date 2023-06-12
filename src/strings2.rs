fn main(){

    //concatenate 2 strings slices without converting it to String

    //1. use format macro

    let hello_string = "Hello";

    let world_string = ", World";

    let hello_world_string = format!("{}{}", hello_string, world_string); // It takes ownership of hello_string and returns it back

    println!("concatenated string is {}", hello_world_string);

    println!("hello_string is {}", hello_string);


    //2. use concatenate method

    let hello_string = "Hello";

    let world_string = ", World";

    let hello_world_string = [hello_string, world_string].concat(); // It takes ownership of hello_string and returns it back

    println!("concatenated string is {}", hello_world_string);

    println!("hello_string is {}", hello_string);

    //3. use + operator

    let hello_string = "Hello";

    let world_string = ", World";

    let hello_world_string = hello_string.to_string() + world_string; // It takes ownership of hello_string and returns it back

    // let hello_world_string = hello_string.to_string() + &world_string; // It takes ownership of hello_string and returns it back

    println!("concatenated string is {}", hello_world_string);


    let hello_string = "Hello";

    let world_string = ", World";

    let hello_world_string = hello_string.to_owned() + world_string; // It takes ownership of hello_string and returns it back

    println!("concatenated string is {}", hello_world_string);

    println!("hello_string is {}", hello_string);


    //4. use push_str

    let mut hello_string = "Hello".to_string();

    let world_string = ", World";

    hello_string.push_str(world_string); // It takes ownership of hello_string and returns it back



    //


    // concatenate 2 strings

    let mut hello_string = "Hello".to_string();

    let world_string = ", World".to_string();

    let hello_world_string = hello_string + &world_string; // It takes ownership of hello_string and returns it back    

    let mut hello_string = "Hello";

    let world_string = ", World";

    let hello_world_string = [hello_string, world_string].concat(); // It takes ownership of hello_string and returns it back

    println!("concatenated string is {}", hello_world_string);

    println!("hello_string is {}", hello_string);  
 
    println!("concatenated string is {}", hello_world_string);

    

    let hello_string = "Hello";

    let chars = hello_string.chars().rev().collect::<String>();

    println!("reverse string is {}", chars);


    //append a char

    let mut hello_str = String::from("hello");

    let new_str = hello_str.push('o');

    let new_str2 = hello_str.push_str(", World");

    println!("new string is {}", hello_str);


    //substring

    let mut hello_str = String::from("hello");

    let first = &hello_str[0..1];

    let second = &hello_str[1..2];


    
    // Iterate over string

    let hello_str = String::from("hello");

    for c in hello_str.chars() {
        println!("{}", c);
    }

    let hello_str = String::from("hello");

    for (i, c) in hello_str.chars().enumerate() {
        println!("{}: {}", i, c);
    }

    let hello_str = String::from("hello");

    let replaced_string = hello_str.replace("ll", "rr");

    println!("replaced string is {}", replaced_string);
    

    let hello_str = String::from("hello");

    let mut replaced_string = String::new();

    let chars = hello_str.chars();

    for c in chars {
        if c == 'l' {
            replaced_string.push('r');
        } else {
            replaced_string.push(c);
        }
    }
}