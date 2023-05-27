// INFO
//  - Variables hold primitive data or references to data
//  - Variable are immutable by default (like const in C++), by default we cannot reassign them
//  - Rust is a block-scoped language

pub fn run(){

    // Create a variable
    let name = "Paulina";
    
    // Create a mutable variable
    let mut age = 25;
    println!("My name is {} and my age was {}", name, age);

    age = 26;
    println!("My name is {} and my age is {}", name, age);

    // Define a constant
    // When a const variable is defined, the type is explicitly needed
    const ID: i32 = 01234;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Paulina", 26);
    println!("My name is {} and my age is {}", my_name, my_age);

}
