fn stream() -> impl Iterator<Item = u32> {
    PrimeIter::new()
}

pub struct PrimeIter {
    curr: u32,
    next: u32,
}

impl PrimeIter {
    fn new() -> Self {
        Self { curr: 2, next: 3 }
    }
}

impl Iterator for PrimeIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.curr;
        self.curr = self.next;
        loop {
            self.next += match self.next % 6 {
                1 => 4,
                _ => 2,
            };
            if is_prime(self.next) {
                break;
            }
        }

        Some(ret)
    }
}

fn is_prime(num: u32) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        n => {
            let n = n as u64;
            let max_p = (n as f64).sqrt().ceil() as u64;
            (5..=max_p)
                .step_by(6)
                .find(|p| n % p == 0 || n % (p + 2) == 0)
                .is_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream().skip(start as usize);

        for i in numbers {
            assert_eq!(
                Some(i),
                prime_iterator.next(),
                "\nYour result (left) did not match the expected output (right)"
            );
        }
    }

    #[test]
    fn timed() {
        let now = Instant::now();
        PrimeIter::new().take(1_000_000).for_each(|_| ());
        println!("Elapsed: {}s", now.elapsed().as_secs_f64())
    }

    #[test]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

        println!("testing segment from 1,000");
        test_segment(
            1_000,
            [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017],
        );
    }
}
