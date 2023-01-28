// Vectors - resizable arrays 

use std::mem; 

pub fn run() {

    let mut numbers: Vec<i32>  = vec![1, 2, 3, 4]; 

    // Re-assign a value in the array 
    // We cannot add on to an array, but we can change the values 
    numbers[2] = 20; 

    // Add on to vector 
    numbers.push(5); 
    numbers.push(6); 

    // pop off the last value (remove 6) 
    numbers.pop(); 

    println!("{:?}", numbers);

    // get a single element/val
    println!("Single Value: {}", numbers[0]);

    // Get vector length 
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated, each element = 4 bytes 
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); 

    // Get slices from array (first two: 0..2)
    let slice: &[i32] = &numbers[0..2]; 
    println!("Slice: {:?}", slice);

    // loop through vector values 
    for x in numbers.iter() {
        println!("Number: {}", x); 
    }

    // loop and mutate values 
    for x in numbers.iter_mut() {
        // multiply each element by 2 
        *x *= 2; 
    }

    println!("Numbers vec {:?}", numbers); 
}


