use std::iter;

fn main(){

    //iterate over strings

    // let hello = String::from("Hello, World");

    // for c in hello.chars(){
    //     println!("{}", c);
    // }

    // iterate over strings using next

    let hello = String::from("Hello, World");

    let mut chars: std::str::Chars = hello.chars();

    assert_eq!(Some('H'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some(','), chars.next());
    assert_eq!(Some(' '), chars.next());
    assert_eq!(Some('W'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('r'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(None, chars.next());

    // iterate over string in reverse order

    let hello = String::from("Hello, World");

    let chars = hello.chars().rev();

    for c in chars{
        println!("{}", c);
    }

    // iterater get list of remaining size

    let hello = String::from("Hello, World");

    let mut chars: std::str::Chars = hello.chars();

    assert_eq!(Some('H'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('l'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some(','), chars.next());
    assert_eq!(Some(' '), chars.next());
    print!("size of remaining chars is {}", chars.size_hint().0);

    

    // iterate over array 

    println!("iterate over array without iterator");

    //In summary, when iterating over a fixed-size array in a for loop, the elements are copied, and ownership of the original array is not transferred. 
    //The Copy trait allows for implicit copying of values, ensuring that the original array remains intact.

    let a = [1,2,3,4,5]; // implements copy trait

    for i in a { // ownership of a is not transferred here because fixed size array implements copy trait
        println!("{}", i);
    }

    let len = a.len();

    println!("length of array is {}", len);

    // iterate over array using iterator

    println!("iterate over array using iterator");

    let a = [1,2,3,4,5];

    let iter = a.iter();

    for i in iter{
        println!("{}", i);
    }


    // iterate over array using iterator and enumerate

    println!("iterate over array using iterator and enumerate");

    let a = [1,2,3,4,5];

    let iter = a.iter().enumerate();

    for (i, j) in iter{
        println!("index {} has value {}", i, j);
    }

    // iterate over array using index

    println!("iterate over array using index");

    let a = [1,2,3,4,5];

    for i in 0..a.len(){
        println!("{}", a[i]);
    }

    //or 

    // let mut index = 0;

    // while(index < a.len()){
    //     println!("{}", a[index]);
    //     index = index + 1;
    // }


    // iterate using iterator next

    println!("iterate using iterator next");

    let a = [1,2,3,4,5];

    let mut iter = a.iter();

    while iter.next() != None {
        iter.next()
    }




    // iterate over string in reverse order

    // let hello = String::from("Hello, World");

    // let  chars = hello.chars().rev();

    // for c in chars{
    //     println!("{}", c);
    // }

    // iterate over string 


    

}