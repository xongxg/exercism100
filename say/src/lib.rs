const ONES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const THOUSANDS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let chunks = chunks(n);
    let mut res = Vec::new();
    for (index, chunk) in chunks.into_iter().enumerate() {
        if chunk == 0 {
            continue;
        }

        res.push(format!("{}", THOUSANDS[index]));
        res.push(below_1000(chunk));
    }

    res.reverse();
    res.join(" ").trim().to_string()
}

fn below_1000(n: u64) -> String {
    match (n / 100, n % 100) {
        (0, k) => below_100(k),
        (n, 0) => format!("{} hundred", ONES[n as usize]),
        (n, k) => format!("{} hundred {}", ONES[n as usize], below_100(k)),
    }
}

fn below_100(n: u64) -> String {
    match (n / 10, n % 10) {
        (0, _) | (1, _) => ONES[n as usize].to_string(),
        (n, 0) => TENS[n as usize].to_string(),
        (n, k) => format!("{}-{}", TENS[n as usize], ONES[k as usize],),
    }
}

fn chunks(mut n: u64) -> Vec<u64> {
    let mut chunks: Vec<u64> = Vec::new();
    while n > 0 {
        let rem = n % 1000;
        n /= 1000;
        chunks.push(rem);
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let input = 0;
        let output = encode(input);
        let expected = "zero";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one() {
        let input = 1;
        let output = encode(input);
        let expected = "one";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn fourteen() {
        let input = 14;
        let output = encode(input);
        let expected = "fourteen";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn twenty() {
        let input = 20;
        let output = encode(input);
        let expected = "twenty";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn twenty_two() {
        let input = 22;
        let output = encode(input);
        let expected = "twenty-two";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn thirty() {
        let input = 30;
        let output = encode(input);
        let expected = "thirty";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn ninety_nine() {
        let input = 99;
        let output = encode(input);
        let expected = "ninety-nine";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_hundred() {
        let input = 100;
        let output = encode(input);
        let expected = "one hundred";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_hundred_twenty_three() {
        let input = 123;
        let output = encode(input);
        let expected = "one hundred twenty-three";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn two_hundred() {
        let input = 200;
        let output = encode(input);
        let expected = "two hundred";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn nine_hundred_ninety_nine() {
        let input = 999;
        let output = encode(input);
        let expected = "nine hundred ninety-nine";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_thousand() {
        let input = 1_000;
        let output = encode(input);
        let expected = "one thousand";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_thousand_two_hundred_thirty_four() {
        let input = 1_234;
        let output = encode(input);
        let expected = "one thousand two hundred thirty-four";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_million() {
        let input = 1_000_000;
        let output = encode(input);
        let expected = "one million";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_million_two_thousand_three_hundred_forty_five() {
        let input = 1_002_345;
        let output = encode(input);
        let expected = "one million two thousand three hundred forty-five";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn one_billion() {
        let input = 1_000_000_000;
        let output = encode(input);
        let expected = "one billion";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn a_big_number() {
        let input = 987_654_321_123;
        let output = encode(input);
        let expected = "nine hundred eighty-seven billion six hundred fifty-four million three hundred twenty-one thousand one hundred twenty-three";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn max_i64() {
        let input = 9_223_372_036_854_775_807;
        let output = encode(input);
        let expected = "nine quintillion two hundred twenty-three quadrillion three hundred seventy-two trillion thirty-six billion eight hundred fifty-four million seven hundred seventy-five thousand eight hundred seven";
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn max_u64() {
        let input = 18_446_744_073_709_551_615;
        let output = encode(input);
        let expected = "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen";
        assert_eq!(output, expected);
    }
}
