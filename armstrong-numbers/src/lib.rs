pub fn is_armstrong_number(num: u32) -> bool {
    // let s = num.to_string();
    // let len = s.clone().len();
    //
    // let sum = s
    //     .chars()
    //     .fold(0, |sum, c| sum as u32 + (c as u32).pow(len as u32));
    //
    // sum == num

    // let len = num.to_string().len();
    // let sum = num.to_string().chars().fold(0, |sum: u32, c| {
    //     sum + c.to_digit(10).unwrap().pow(len as u32)
    // });
    //
    // sum == num

    let len = num.to_string().len();
    let sum = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |sum, x| sum + x.pow(len as u32));

    sum == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    #[ignore]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    #[ignore]
    fn there_are_no_two_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    #[ignore]
    fn three_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    #[ignore]
    fn three_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    #[ignore]
    fn four_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_474))
    }

    #[test]
    #[ignore]
    fn four_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_475))
    }

    #[test]
    #[ignore]
    fn seven_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }

    #[test]
    #[ignore]
    fn seven_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_926_314))
    }
}
