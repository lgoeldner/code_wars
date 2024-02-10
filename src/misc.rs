use core::num;
use std::{clone, cmp::Ordering, collections::HashMap, iter::zip, rc::Rc, sync::Mutex};

/// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust
pub fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() & 1 {
        0 => "even".to_owned(),
        1 => "odd".to_owned(),
        _ => panic!("this doesn't happen!"),
    }
}

/// https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust
pub fn digital_root(n: i64) -> i64 {
    fn get_digits(num: u64) -> Vec<u64> {
        num.to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as u64)
            .collect()
    }

    fn root_sum(numbers: Vec<u64>) -> u64 {
        if numbers.len() == 1 {
            return numbers[0];
        }

        let digits_vec = dbg!(get_digits(numbers.iter().sum()));
        root_sum(digits_vec)
    }

    root_sum(get_digits(n.try_into().unwrap())) as i64
}

#[test]
fn test_digital_root() {
    assert_eq!(digital_root(16), 7);
}

fn fib(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

/// https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() <= k {
        return strarr.join("");
    }

    let mut first_len = 0;
    let mut ending = 0;

    for i in strarr.windows(k) {
        let first_elem_len = i.first().unwrap().len();
        let last_elem_len = i.last().unwrap().len();
        dbg!(i);
    }

    todo!()
}

fn test_helper_lconsec(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    test_helper_lconsec(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    test_helper_lconsec(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    test_helper_lconsec(vec![], 3, "");
    test_helper_lconsec(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    test_helper_lconsec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    test_helper_lconsec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

/// https://leetcode.com/problems/concatenation-of-array/
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut clone = nums.clone();
    let mut result = nums.clone();
    result.append(&mut clone);
    result
}

/// https://leetcode.com/problems/length-of-last-word/description/
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .rev()
        .next()
        .map(|word| word.len())
        .unwrap_or(0) as i32
}

pub fn recursive_stack_direction<T: Default>(ptr: Option<*const T>) -> Ordering {
    // allocate a variable and make a raw pointer to it
    let num: T = Default::default();
    let numptr: *const T = &num;
    match ptr {
        // recursively call the function with the raw pointer
        None => recursive_stack_direction(Some(numptr)),
        // if there is a pointer passed in, compare the raw pointers
        Some(ptr) => numptr.cmp(&ptr),
    }
}

#[test]
fn test_() {
    match recursive_stack_direction::<usize>(None) {
        Ordering::Equal => panic!("this doesn't happen!"),
        Ordering::Greater => println!("The stack grows upwards"),
        Ordering::Less => println!("The stack grows downwards"),
    }
}
