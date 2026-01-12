# Rust Logic Building – Intermediate

1. **Vector Ownership and Filtering**
    Write a function `filter_integers` that takes ownership of a `Vec<i32>`. The function should create and return a new vector containing only the positive integers from the input, preserving their original order. The original vector should be consumed by the function.
    *Input:* A vector of integers (e.g., `[10, -5, 3, 0, -2]`).
    *Output:* A new vector `[10, 3]`.

2. **String Slice Analysis**
    Create a function that accepts a string slice (`&str`) as an argument. The function should iterate through the characters and count the number of vowels (a, e, i, o, u) using pattern matching, ignoring case. Return the total count.
    *Input:* `"Rust Programming"`
    *Output:* `4`

3. **In-Place Vector Reversal**
    Write a function that takes a mutable reference to a vector of integers (`&mut Vec<i32>`). Implement logic to reverse the elements of the vector **in-place** (without creating a new vector). You must swap elements using indices.
    *Input:* A mutable vector `[1, 2, 3, 4]`.
    *Output:* The vector is modified to `[4, 3, 2, 1]`.

4. **Tuple Pattern Matching (FizzBuzz)**
    Write a function that takes an integer `n` and returns a `String`. Inside the function, create a tuple `(n % 3, n % 5)` and use a `match` expression on this tuple to return "Fizz" for multiples of 3, "Buzz" for multiples of 5, "FizzBuzz" for multiples of both, and the number as a string otherwise.
    *Input:* `15`
    *Output:* `"FizzBuzz"`

5. **Merging Sorted Vectors**
    Write a function that accepts references to two sorted vectors of integers (`&Vec<i32>`, `&Vec<i32>`). The function should verify which elements are smaller and merge them into a single, new sorted vector representing the combination of both inputs. Do not just concatenate and sort; use a logic-based merge approach.
    *Input:* `[1, 4, 6]` and `[2, 3, 5]`
    *Output:* `[1, 2, 3, 4, 5, 6]`

6. **Word Reverser**
    Create a function that takes ownership of a `String` containing words separated by spaces. The function should split the string into a vector of words, reverse the order of the words (but not the characters within the words), and join them back into a single `String` to return.
    *Input:* `"Hello Rust World"`
    *Output:* `"World Rust Hello"`

7. **Find Maximum in Slice**
    Write a function that accepts a slice of integers `&[i32]`. Use a loop and a comparison variable to find and return the largest value in the slice. Ensure your code handles both positive and negative numbers correctly.
    *Input:* `[-10, -5, -20, -2]`
    *Output:* `-2`

8. **Anagram Checker (Sorting)**
    Write a function that accepts two string slices (`&str`). The function should determine if the two strings are anagrams (contain the exact same characters with the same frequency). To do this, convert the strings to vectors of characters, sort the vectors, and compare them for equality.
    *Input:* `"silent"` and `"listen"`
    *Output:* `true`

9. **Run-Length Encoding**
    Write a function that takes a reference to a `String`. Iterate through the characters to produce a new `String` that represents the run-length encoding of the input. For example, consecutive repeated characters should be replaced by the character followed by the count.
    *Input:* `"AAABBBCC"`
    *Output:* `"A3B3C2"`

10. **Vector Deduplication**
    Write a function that takes a mutable reference to a sorted `Vec<i32>`. Iterate through the vector and remove any duplicate consecutive elements so that each number appears only once. Do this by modifying the vector directly.
    *Input:* A mutable vector `[1, 1, 2, 2, 2, 3]`
    *Output:* The vector becomes `[1, 2, 3]`
