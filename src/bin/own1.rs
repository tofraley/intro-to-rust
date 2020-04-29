// For clarity, don't show these warnings when building
#![allow(dead_code, unused_variables)]

fn main() {
    // Add some space in the console for clarity
    println!("{}", "\n-----Start of demo 1!-----\n\n");

    // When values are simple with a known, fixed size
    // they will be copied.
    // These integers work how you'd expect.
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // A string won't work the same way.
    let s1 = String::from("hello"); // Here we create a pointer to String.

    // let s2 = s1; // Here the pointer to the String is moved.
    // println!("s1: {}", s1); // Here we try to use s1, but it is invalid.

    // let s2 = s1.clone(); // let's explicitly deep copy s1
    // println!("s1 = {}, s2 = {}", s1, s2); // success!

    // Add some space in the console for clarity
    println!("{}", "\n\n-----End of demo 2!-----\n\n");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
