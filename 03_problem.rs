/*
Write a function that takes a mutable reference to a vector of integers (`&mut Vec<i32>`). 
Implement logic to reverse the elements of the vector **in-place** 
(without creating a new vector). You must swap elements using indices.
*Input:* A mutable vector `[1, 2, 3, 4]`.
*Output:* The vector is modified to `[4, 3, 2, 1]`.
*/

fn main() {
    let mut v = vec![1, 2, 3, 4];
    reverse_in_place(&mut v);
    println!("{:?}", v);
}

fn reverse_in_place(v: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = v.len() - 1;
    
    while left < right {
        v.swap(left, right);
        left += 1;
        right -= 1;
    }
}
