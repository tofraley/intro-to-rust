// For clarity, don't show these warnings when building
#![allow(dead_code, unused_variables)]

// Functions work the same way.
// Simple values are copied into or out of functions.
// Values that include data on the heap are moved.
// Unless it's values have moved, they will be dropped when their owner variable goes out of scope.

fn main() {
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // Add some space in the console for clarity
    println!("{}", "\n\n-----End of demo 2!-----\n\n");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
