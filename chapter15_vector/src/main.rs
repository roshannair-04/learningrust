fn main() {
    // ----------------- VECTORS -----------------

    // // 1. Creating an empty vector manually
    // let mut v: Vec<i32> = Vec::new();

    // // 2. Using the `vec!` macro with initial values
    // let mut v = vec![1, 2, 3];

    // // Pushing elements to the end (requires `mut`)
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // v.push(9);

    // println!("{v:?}");

    // // Resetting for indexing demonstration
    // let v = vec![1, 2, 3, 4, 5];

    // // Access Method 1: Direct indexing (panics if index is out of bounds)
    // let third: &i32 = &v[2]; // index 2 = third element
    // println!("The third element is {third}");

    // // Access Method 2: .get() method (returns Option<&T>, safe against panics)
    // let third = v.get(2);
    // match third {
    //     Some(val) => println!("The third element via .get() is {val}"),
    //     None => println!("There is no third element."),
    // }

    // ------------------------------UTF-8---------------------------------
// 1 & 2: Creating heap-allocated Strings from string literals (&str)
    // let s = "whatever".to_string();
    // let s = String::from("whatever");

    // // Appending: push_str takes a string slice (&str), push takes a single character ('char')
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');
    // println!("the value of s = {s}"); // "foobar!"

    // // UTF-8 multi-byte strings (each Cyrillic character takes 2 bytes, not 1)
    // let salam = String::from("Здравствуйте");
    // let salut = String::from("Salut");

    // // Concatenation via `+`: takes ownership of `s1` and borrows `s2`
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // s1 is moved; cannot be used after this line
    // println!("The value of s3 = {s3}");

    // // format! macro: concatenates without taking ownership of any variables
    // let full_message = format!("{salam} {salut}");
    // println!("{full_message}");


    // ------------------------------HASHMAPS-----------------------------------------
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
        println!("{key}: {value}");
    }



}