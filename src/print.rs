pub fn run() {
    // Print to console 
    println!("Hello from the print.rs file"); 

    // Basic Formatting 
    println!("{} is from {}", "Brad", "Mass"); // Print integer with string literal 

    // Positional Arguments 
    println!("{0} is from {1} and {0} likes to {2}", 
    "Brad", "Mass", "code"); 

    // Named arguments 
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball"); 

    // Placeholder traits 
    println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10); 

    // Placeholder for debug traits, comes in handy for printing an entire array ie. 
    println!("{:?}", (12, true, "hello")); 

    // Basic math 
    println!("10 + 10  = {}", 10 + 10); 
}