#[allow(dead_code, unused_variables)]
// Taking ownership and then returning ownership with every function is a bit tedious.
// What if we want to let a function use a value but not take ownership?
// We would use a reference.
// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
// Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
// When a function has a reference as a parameter, we call it 'borrowing'.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // References are immutable by default,
    // so trying to change it won't work
    change_immut(&s1);

    //let mut s2 = String::from("hello");
    //change_mut(&mut s2);fn main() {

    // But mutable references have one big restriction:
    // you can have only one mutable reference to a
    // particular piece of data in a particular scope.
    // This code will fail:
    //let r1 = &mut s2;
    //let r2 = &mut s2;

    //println!("{}, {}", r1, r2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// this will cause an error
// because references are immutable by default
fn change_immut(some_string: &String) {
    some_string.push_str(", world");
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}
