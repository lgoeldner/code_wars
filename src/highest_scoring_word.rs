//! https://www.codewars.com/kata/57eb8fcdf670e99d9b000272/train/rust
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
