//! https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/rust

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
