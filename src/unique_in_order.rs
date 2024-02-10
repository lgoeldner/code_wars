//! https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
	T: std::iter::IntoIterator,
	T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
	let mut iterw = sequence.into_iter().peekable();
	let mut res_vec = vec![];
	while let Some(item) = iterw.next() {
		if iterw.peek() != Some(&item) {
			res_vec.push(item);
		}
	}

	res_vec
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_test() {
		assert_eq!(
			unique_in_order("AAAABBBCCDAABBB".chars()),
			vec!['A', 'B', 'C', 'D', 'A', 'B']
		);
	}
}
