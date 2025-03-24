/*
using some simple logic: create the number. slap the digits of the number into a Vec.
using the definition of a palindrome, check if the reversed array is the same as the original. then put out the biggest value
*/


use std::iter::successors;

fn main() {
    find_palindromes_simplified(100, 999);
}

// credit: https://codereview.stackexchange.com/a/226357, https://stackoverflow.com/a/69302957/23320053
fn unwrap_palindrome(num: i64) -> bool {
    let mut num = num;
    let mut arr: Vec<i64> = vec![];
    let num_len = successors(Some(num), |&num| (num >= 10).then(|| num / 10)).count();
    for _ in 1..=num_len {
        arr.push(num % 10);
        num /= 10;
    }
    // credit for reversing array: https://users.rust-lang.org/t/reversing-an-array/44975/2
    let rev_arr: Vec<i64> = arr.iter().copied().rev().collect();
    if rev_arr == arr {
        return true;
    }
    return false;
}

fn find_palindromes_simplified(start: i64, end: i64) {
    let mut collector: Vec<i64> = vec![];
    for i in start..=end {
        for j in start..=end {
            let num = i * j;
            if unwrap_palindrome(num) {
                collector.push(num);
            }
        }
    }
    let final_value = find_max_value(collector);
    println!("{}", final_value)
}

fn find_max_value(arr:Vec<i64>) -> i64 {
    let mut tracker:i64 = 0;
    for i in 1..arr.len() {
        if arr[i] > tracker {
            tracker = arr[i];
        }
    }
    return tracker;
}