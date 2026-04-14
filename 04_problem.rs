/*
Write a function that takes an integer `n` and returns a `String`. 
Inside the function, create a tuple `(n % 3, n % 5)` and use a `match` 
expression on this tuple to return "Fizz" for multiples of 3, "Buzz" for 
multiples of 5, "FizzBuzz" for multiples of both, and the number as a 
string otherwise.
*Input:* `15`
*Output:* `"FizzBuzz"`
*/

fn main() {
    println!("{}", fizzbuzz(15));
    println!("{}", fizzbuzz(9));
    println!("{}", fizzbuzz(5));
    println!("{}", fizzbuzz(7));
}

fn fizzbuzz(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}
