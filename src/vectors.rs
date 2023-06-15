use std::vec;

fn main(){

    //how to create vector
    let v1 = vec![1,2,3,4,5];

    let mut v2 = Vec::new();

    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);



    //access

    let first = &v1[0]; // may create a runtime panic if you try to access an element outside the bounds of the vector

    let second = v1.get(1).unwrap(); 

    let third = v1.get(3); // This is more safer approach as it returns an Option<T> type

    match third {
        Some(i) => println!("Third element is {}", i),
        None => println!("No third element")
    }

    let four = v1[4]; //copies the element. Ownershup not moved

    println!("Fourth element is {}", four); // This will create a runtime panic
    println!("length of vector is {}", first);

//    println!("First element is {}", v1);


    //modify

    let mut v3 = vec![1,2,3,4,5];

    let mut first = &mut v3[0];

    *first = 10;

    println!("First element is {}", first);

    println!("Vector is {:?}", v3);

    
    v3[0] = 20;

    println!("Vector is {:?}", v3);


    //remove


    //Using remove method
    let mut v4 = vec![1,2,3,4,5];

    let first = v4.remove(0); // o is the index

    println!("First element is {}", first);

    println!("Vector is {:?}", v4);

    //using drain method

    let mut v5 = vec![1,2,3,4,5];

    let drained = v5.drain(0..=1);// 0..1 is the range

    for i in drained {
        println!("Drained element is {}", i);
    }   


    println!("First element is {}", first);

    println!("Vector is {:?}", v5);

    //using pop method

    let mut v6 = vec![1,2,3,4,5];

    let last = v6.pop();

    println!("Last element is {:?}", last);

    println!("Vector is {:?}", v6);

    // using clear method

    let mut v7 = vec![1,2,3,4,5];

    v7.clear();

    println!("Vector is {:?}", v7);

    // using truncate method

    let mut v8 = vec![1,2,3,4,5];

    v8.truncate(2);

    println!("Vector is {:?}", v8);

    //using retain method

    let mut v9 = vec![1,2,3,4,5];


    v9.retain(|&x| x % 2 == 0);

    println!("Vector is {:?}", v9);

    // using resize method

    let mut v10 = vec![1,2,3,4,5];

    v10.resize(2, 0);

    println!("Vector is {:?}", v10);

   // Iteration

   let v11 = vec![1,2,3,4,5];
 
   for i in v11 { // This will move the ownership of the vector
       println!("Element is {}", i);
   }
    

   let v12 = vec![1,2,3,4,5];

    for i in &v12 { // This will not move the ownership of the vector
         println!("Element is {}", i);
    }

    let mut v13 = vec![1,2,3,4,5];

    for i in &mut v13 { // This will not move the ownership of the vector
         *i += 10;
    }

    println!("Vector is {:?}", v13);


    // Iterator always return reference to the element. It can be mutable or immutable. In case you want to chaneg the value it points to you need to use dereference operator *

    let iter = v13.iter();

    for i in iter {
        println!("Element is {}", i);
    }

    //using mutable iterator

    let mut iter = v13.iter_mut();

    for i in iter {
        *i += 10;
    }

    let mut i = 0;

    while  i < v13.len() {
        println!("Element is {}", v13[i]); // Here you can just iterate over the value and not the reference
        i += 1;
    }


    //convert array to vector

    let arr = [1,2,3,4,5];

    let v14 = arr.to_vec();

    println!("Vector is {:?}", v14);



    //convert vector to slice

    let mut v15 = vec![1,2,3,4,5];

    let slice = &v15[..];

    let slice2 = &mut v15[..]; 

    // println!("Slice is {:?}", slice);

    let mut v16 = vec![1,2,3,4,5];

    // let slice = v16.as_slice();
    
    let mut_slice = v16.as_mut_slice();

    mut_slice[0] = 10;

    println!("Vectoris {:?}", v16);

    //convert vector to string

    let v17 = vec![1,2,3,4,5];

    let s = format!("{:?}", v17);

    println!("String is {}", s);

    // split vector into two at index 2

    let mut v18 = vec![1,2,3,4,5];

    let v19 = v18.split_off(2);

    println!("Vector 1 is {:?}", v18);

    println!("Vector 2 is {:?}", v19);

    //split vector into two at index 2

    let mut v20 = vec![1,2,3,4,5];

    let v21 = v20.split_at_mut(2);

    println!("Vector 1 is {:?}", v21.0);

    println!("Vector 2 is {:?}", v21.1);

    v21.0[0] = 10;

    println!("Vector 1 is {:?}", v21.0);

    println!("Vector 2 is {:?}", v21.1);

    //split vector into two at index 2

    let mut v22 = vec![1,2,3,4,5];

    let v23 = v22.split_at(2);

    println!("Vector 1 is {:?}", v23.0);

    println!("Vector 2 is {:?}", v23.1);

    





    



    

}