#![allow(unused)]

mod highest_scoring_word;
mod human_readable_time;
mod array_diff;
mod valid_paren;

/// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust
fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() & 1 {
        0 => "even".to_owned(),
        1 => "odd".to_owned(),
        _ => panic!("this doesn't happen!"),
    }
}

