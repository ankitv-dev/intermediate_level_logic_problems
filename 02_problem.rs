/*
Create a function that accepts a string slice (`&str`) as an argument. 
The function should iterate through the characters and count the 
number of vowels (a, e, i, o, u) using pattern matching, ignoring 
case. Return the total count.
*Input:* `"Rust Programming"`
*Output:* `4`
*/

fn main() {
    let st = "Rust";
    println!("{}", vowel_count(st))
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