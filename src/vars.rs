// Variables hold primitive data or references to data 
// Variables are immutable by default (cant be reassigned)
// Rust is a block-scoped language 

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {} years old", name, age);
    
    age = 38; // increment 
    println!("My name is {} and I am {} years old", name, age);

    // Define constant (usually UPPERCASE)
    const ID: i32 = 001; 
    println!("ID: {}", ID);

    // Assign multiple cars 
    let (my_name, my_age ) = ("Brad", 37); 
    println!("{} is {}", my_name, my_age);

}
