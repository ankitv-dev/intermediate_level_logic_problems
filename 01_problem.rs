/*
Write a function `filter_integers` that takes ownership of a `Vec<i32>`. 
The function should create and return a new vector containing only the 
positive integers from the input, preserving their original order. 
The original vector should be consumed by the function.
*Input:* A vector of integers (e.g., `[10, -5, 3, 0, -2]`).
*Output:* A new vector `[10, 3]`.
*/

fn main() {
    let v = vec![10, -5, 3, 0, -2];
    println!("{:?}", filter_integers(v));
}

fn filter_integers(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().filter(|n| *n > 0).collect()
}
