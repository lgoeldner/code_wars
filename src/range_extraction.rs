
use std::fmt::Write;
pub fn range_extraction(a: &[i32]) -> String {
    // Your solution here
    let mut range_start = None;
    let mut last = a.first().unwrap();
    let mut buf = String::with_capacity(a.len());
    let mut str_iter = a.into_iter().skip(1);

    while let Some(element) = str_iter.next() {
        if range_start.is_none() && *element == last + 1 {
            range_start = Some(*element);
        } else if range_start.is_some() && *element == last + 1 {
            last = element;
        } else if range_start.is_some() && *element != last + 1 {
            if range_start.unwrap() == *last {
                write!(buf, "{last},");
                range_start = Some(*element);
            } else {
                write!(buf, "{0}-{last},", range_start.unwrap());
            }
        } else {
            // write!(buf, "{},", element);
            range_start = Some(*element);
        }

        last = element;
    }

    buf.pop();
    buf.shrink_to_fit();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
