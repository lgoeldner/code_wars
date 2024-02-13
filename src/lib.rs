#![allow(unused)]

mod array_diff;
mod highest_scoring_word;
mod human_readable_time;
mod misc;
mod more;
mod unique_in_order;
mod valid_paren;
mod range_extraction;

/// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust
fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() & 1 {
        0 => "even".to_owned(),
        1 => "odd".to_owned(),
        _ => panic!("this doesn't happen!"),
    }
}
