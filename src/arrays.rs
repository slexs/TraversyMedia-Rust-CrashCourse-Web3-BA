// Arrays: fixed list where elements are the same data types 
use std::mem; 

pub fn run() {
    // let numbers: [i32; 5]  = [1, 2, 3, 4]; // ERR: too short, found 4, expected 5 
    // let numbers: [i32; 5]  = [1, 2, 3, 4, "Hello"]; // ERR: expected i32, found string 
    // let mut numbers: [i32; 5]  = [1, 2, 3, 4, 5]; 
    let mut numbers: [i32; 4]  = [1, 2, 3, 4]; 

    // Re-assign a value in the array 
    // We cannot add on to an array, but we can change the values 
    numbers[2] = 20; 

    println!("{:?}", numbers);

    // get a single element/val
    println!("Single Value: {}", numbers[0]);

    // Get array length 
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated, each element = 4 bytes 
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); 

    // Get slices from array (first two: 0..2)
    let slice: &[i32] = &numbers[0..2]; 
    println!("Slice: {:?}", slice);
}


