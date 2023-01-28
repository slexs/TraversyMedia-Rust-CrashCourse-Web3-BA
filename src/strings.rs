// Primitive str = Immutable fixed-length string somewhere in memory 
// String = Growable, heap-allocated data structure
// ^ Use when you need to modify or own string data 

pub fn run() {
    // let hello = "Hello"; // Immutable fixed length (Primitive str) 
    let mut hello = String::from("Hello "); // Growable, heap-allocated data structure

    // Get string length 
    println!("Length: {}", hello.len());

    // push char 
    hello.push('W'); 

    // push string
    hello.push_str("orld"); 

    // Get caapacity (num of bytes that it can store) 
    println!("Capacity: {}", hello.capacity()); 

    // check if string empty? 
    println!("Is Empty: {}", hello.is_empty()); 

    // Check for substring 
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace 
    println!("Replace: {}", hello.replace("World", "There")); 

    // Loop through string by whitespace 
    for word in hello.split_whitespace() {
        println!("{}", word); 
    }

    // Create string with specified capacity 
    let mut s = String::with_capacity(10); 
    s.push('a');
    s.push('b');
    
    println!("{}", s); 

    // Assertion testing (R eq L?) 
    assert_eq!(2, s.len()); 
    assert_eq!(10, s.capacity()); 

    println!("{}", hello);
}