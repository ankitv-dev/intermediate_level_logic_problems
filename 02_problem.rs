/*
Create a function that accepts a string slice (`&str`) as an argument. 
The function should iterate through the characters and count the 
number of vowels (a, e, i, o, u) using pattern matching, ignoring 
case. Return the total count.
*Input:* `"Rust Programming"`
*Output:* `4`
*/

use std::io;
use std::io::Write;

fn main() {
    print!("Enter your word: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    println!("{}", vowel_count(&input))
}

fn vowel_count(s:&str)->i32{
    let mut count = 0;

    for ch in s.chars() {
        match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => {}
        }
    }

    count
}