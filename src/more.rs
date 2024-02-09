use std::ops::Sub;

mod rot13 {
    //! https://www.codewars.com/kata/530e15517bc88ac656000716/train/rust
    fn rot13(message: &str) -> String {
        const fn rot13_process(ch: char) -> char {
            let offset = match ch.is_ascii_lowercase() {
                true => 97,
                false => 65,
            };
            ((ch as u8 - offset + 13) % 26 + offset) as char
        }

        let mut result = String::with_capacity(message.len());
        for ch in message.chars() {
            if !ch.is_ascii_alphabetic() {
                result.push(ch);
                continue;
            }
            let result_ch = rot13_process(ch);
            result.push(result_ch);
        }
        result
    }

    // const fn rot13_process(ch: char) -> char {
    //     ((ch as u8 + 13) % 26) as char
    // }

    #[cfg(test)]
    mod tests {
        use super::rot13;

        const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

        fn dotest(s: &str, expected: &str) {
            assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
        }

        #[test]
        fn sample_tests() {
            dotest("test", "grfg");
            dotest("Test", "Grfg");
        }
    }
}

#[test]
fn test_char_to_index() {
    fn rot13(ch: char) -> char {
        let mut ch = ch as u8;
        let offset = if ch.is_ascii_lowercase() { 97 } else { 65 };
        ch -= offset;
        ch += 13;
        ch %= 26;
        ch += offset;

        dbg!(ch as char)
    }

    rot13('t');
    rot13('e');
    rot13('s');
    rot13('t');
}

pub const fn function<T: Copy, const F: usize>(a: T) -> [T; F] {
    [a; F]
}
