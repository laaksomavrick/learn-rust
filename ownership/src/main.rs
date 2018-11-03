fn main() {
    {
        // s is allocated
        let mut s = String::from("I'm on the heap!");
        s.push_str(" And therefore I can be an unknown size of memory at compile time!");
        println!("{}", s);
    } // s is freed

    let x = 5; // value type
    let y = x; // value type

    let s1 = String::from("reference type");
    let s2 = s1; // ptr to same thing on heap
                 // this actually performs a "move" -> s1 is invalid now
                 // println!("{}", s1); invalid
                 // println!("{}", s2); valid
    let s3 = String::from("I'm a little teapot");
    let s4 = s3.clone();
    // deep copy

    let s5 = String::from("asdasd");
    takes_ownership(s5);
    // s5 no longer valid, has been freed
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// to avoid tediousness of taking and giving ownership, we can use reference types
// calculate length won't take ownership of s
// this is referred to as borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

// this won't work - we can't borrow and mutate something
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// this will work
// restriction: you can only have one mutable reference to a piece of data in particular scope. E.g this will fail:
// let mut s = String::from("asd");
// let r1 = &mut s;
// let r2 = &mut s;

// to fix
// let mut s = String::from("asd");
// {
//     let r1 = &mut s;
// }
// let r2 = &mut s;
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
