//! https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust

const SECONDS_IN_HOUR: u32 = 3600;
const SECONDS_IN_MINUTE: u32 = 60;

fn make_readable(seconds: u32) -> String {
    let mut seconds = seconds;

    let hours = seconds / SECONDS_IN_HOUR;
    seconds %= SECONDS_IN_HOUR;

    let minutes = seconds / SECONDS_IN_MINUTE;
    seconds %= SECONDS_IN_MINUTE;

    format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
}

#[cfg(test)]
mod tests {
    use super::make_readable;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }
}
