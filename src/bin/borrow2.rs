#[allow(dead_code, unused_variables)]
// The benefit of having this restriction is that
// Rust can prevent data races at compile time.
// A data race is similar to a race condition and
// happens when these three behaviors occur:
// - Two or more pointers access the same data at the same time.
// - At least one of the pointers is being used to write to the data.
// - There’s no mechanism being used to synchronize access to the data.

fn main() {
    // We can have multiple mutable references,
    // just not simultaneous ones
    let mut s1 = String::from("hello");

    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s1;

    // We also cannot have a mutable reference while
    // we have an immutable one.

    let mut s2 = String::from("hello");

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    let r3 = &mut s2; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    // This code will compile because the last usage
    // of the immutable references occurs before the
    // mutable reference is introduced:

    let mut s3 = String::from("hello");

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s3; // no problem
    println!("{}", r3);

}
