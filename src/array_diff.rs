//! https://www.codewars.com/kata/523f5d21c841566fde000009/rust

fn array_diff<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut temp_vec: Vec<T> = Vec::new();
    // for (i, item) in a.iter().enumerate() {
    //     if !b.contains(&item) {
    //        temp_vec.push(item);
    //     }
    // }
    a.into_iter().filter(|x| !b.contains(x)).collect()
}
