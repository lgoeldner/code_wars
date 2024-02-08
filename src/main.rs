#![allow(unused)]
fn main() {
    println!("Hello, world!");
}
mod highest_scoring_word {

    /// https://www.codewars.com/kata/57eb8fcdf670e99d9b000272/train/rust
    fn high(input: &str) -> &str {
        input
            .split_ascii_whitespace()
            .fold((0, ""), |acc, word| {
                let (score, _) = dbg!(get_score(word), word);
                if score > acc.0 {
                    (score, word)
                } else {
                    acc
                }
            })
            .1
        // .max_by_key(get_score)
    }

    fn get_score(word: &str) -> u32 {
        word.bytes().fold(0, |acc: u32, ch| {
            acc + (ch.to_ascii_uppercase() - 64) as u32
        })
    }

    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }

    #[test]
    fn word_order() {
        assert_eq!("ginner", high("ginner nniger"))
    }

    #[test]
    fn score() {
        assert_eq!(8, get_score("abad"));
    }
}

/// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust
fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() & 1 {
        0 => "even".to_owned(),
        1 => "odd".to_owned(),
        _ => panic!("this doesn't happen!"),
    }
}

/// https://www.codewars.com/kata/523f5d21c841566fde000009/rust
mod array_diff {
    fn array_diff<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let mut temp_vec: Vec<T> = Vec::new();
        // for (i, item) in a.iter().enumerate() {
        //     if !b.contains(&item) {
        //        temp_vec.push(item);
        //     }
        // }
        a.into_iter().filter(|x| !b.contains(x)).collect()
    }
}

/// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust
mod human_readable_time {
    const SECONDS_IN_HOUR: u32 = 3600;
    const SECONDS_IN_MINUTE: u32 = 60;

    fn make_readable(seconds: u32) -> String {
        let mut seconds = seconds;

        let hours = seconds / SECONDS_IN_HOUR;
        seconds %= SECONDS_IN_HOUR;

        let minutes = seconds / SECONDS_IN_MINUTE;
        seconds %= SECONDS_IN_MINUTE;

        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    }

    #[cfg(test)]
    mod tests {
        use super::make_readable;

        const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

        fn dotest(s: u32, expected: &str) {
            assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
        }

        #[test]
        fn fixed_tests() {
            dotest(0, "00:00:00");
            dotest(59, "00:00:59");
            dotest(60, "00:01:00");
            dotest(3599, "00:59:59");
            dotest(3600, "01:00:00");
            dotest(86399, "23:59:59");
            dotest(86400, "24:00:00");
            dotest(359999, "99:59:59");
        }
    }
}

/// https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/rust
mod valid_paren {
    use std::error::Error;

    #[derive(Debug, Clone, Copy)]
    struct Paren {
        style: ParenStyle,
        is_closing: bool,
    }

    impl From<char> for Paren {
        fn from(value: char) -> Self {
            let (is_closing, style) = match value {
                '(' => (false, ParenStyle::Normal),
                ')' => (true, ParenStyle::Normal),

                '{' => (false, ParenStyle::Curly),
                '}' => (true, ParenStyle::Curly),

                '[' => (false, ParenStyle::Bracket),
                ']' => (true, ParenStyle::Bracket),

                _ => panic!("Invalid char: {value}"),
            };
            Self { is_closing, style }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ParenStyle {
        Normal,
        Bracket,
        Curly,
    }

    fn valid_braces(s: &str) -> bool {
        let mut stack: Vec<Paren> = Vec::new();

        for ch in s.chars() {
            let curr = dbg!(Paren::from(ch));
            if curr.is_closing {
                if stack.is_empty() {
                    return false;
                }

                let top = stack.last().expect("stack is empty (should never happen)");

                if top.style != curr.style {
                    return false;
                }

                stack.pop();
            } else {
                stack.push(curr);
            }
        }

        stack.is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            expect_false("(})");
        }

        #[test]
        fn basic_tests() {
            expect_true("()");
            expect_false("[()])");
        }

        fn expect_true(s: &str) {
            assert!(
                valid_braces(s),
                "Expected {s:?} to be valid. Got false",
                s = s
            );
        }

        fn expect_false(s: &str) {
            assert!(
                !valid_braces(s),
                "Expected {s:?} to be invalid. Got true",
                s = s
            );
        }
    }
}
