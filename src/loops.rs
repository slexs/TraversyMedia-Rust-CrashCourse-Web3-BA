// Loops - used to iterate until a condition is met 

pub fn run() {
    let mut count = 0; 

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break; 
    //     }
    // }

    // While loop (FizzBuzz), loop through 0 to 100
    // if num is div by 3: print out fizz 
    // if num is div by 5: print out buzz
    // if num is div by both: print out fizzbuzz 
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("Fizzbuzz"); 
    //     }
    //     else if count % 3 == 0 {
    //         println!("fizz"); 
    //     }
    //     else if count % 5 == 0  {
    //         println!("buzz")
    //     } else { println!("{}", count);}

    //     // increment 
    //     count += 1; 
    // }

    // For range 
    for x in 0..100 {
        if x % 15 == 0 {
            println!("Fizzbuzz"); 
        }
        else if x % 3 == 0 {
            println!("fizz"); 
        }
        else if x % 5 == 0  {
            println!("buzz")
        } else { println!("{}", x);}
    }


}