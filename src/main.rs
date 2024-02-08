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

}
