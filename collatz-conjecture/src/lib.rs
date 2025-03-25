pub fn collatz(n: u64) -> Option<u64> {
    // if n < 1 {
    //     return None;
    // }
    //
    // let mut n = n;
    // let mut steps: u64 = 0;
    //
    // while n > 1 {
    //     match n % 2 == 0 {
    //         true => n /= 2,
    //         false => n = n * 3 + 1,
    //     }
    //
    //     steps += 1;
    // }
    //
    // Some(steps)
    collatz_next(n,0)
}

fn collatz_next(n: u64, steps: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(steps),
        _ => collatz_next(
            match n % 2 {
                0 => n / 2,
                _ => n * 3 + 1,
            },
            steps + 1,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_steps_for_one() {
        let output = collatz(1);
        let expected = Some(0);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn divide_if_even() {
        let output = collatz(16);
        let expected = Some(4);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn even_and_odd_steps() {
        let output = collatz(12);
        let expected = Some(9);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn large_number_of_even_and_odd_steps() {
        let output = collatz(1_000_000);
        let expected = Some(152);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn zero_is_an_error() {
        let output = collatz(0);
        let expected = None;
        assert_eq!(output, expected);
    }
}
