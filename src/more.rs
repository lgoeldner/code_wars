mod rot13 {
    //! https://www.codewars.com/kata/530e15517bc88ac656000716/train/rust
    fn rot13(message: &str) -> String {
        for ch in message.chars() {
            if !ch.is_ascii_alphabetic() {
                continue;
            }
            // convert char to index in alphabet
            // if ch.is_ascii_lowercase() {
            //     let index = rot13_process(ch);
            //     dbg!(index as char);
            // } else {
            //     let index = ((ch as u8 + 13) % 26) as char;
            //     dbg!(index as char);
            // }

            let index = rot13_process(ch);
            dbg!(index as char);
        }
        todo!()
    }

    const fn rot13_process(ch: char) -> char {
        ((ch as u8 + 13) % 26) as char
    }

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
    fn char
}