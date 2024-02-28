//! https://www.codewars.com/kata/523f5d21c841566fde000009/rust
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a
    .into_iter()
    .filter(|i| !b.contains(i))
    .collect()
}