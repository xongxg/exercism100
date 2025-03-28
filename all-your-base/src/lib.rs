#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn require<T>(predict: bool, error: T) -> Result<(), T> {
    if predict { Ok(()) } else { Err(error) }
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    convert_from_base(number, from_base).and_then(|n| convert_to_base(n, to_base))
}

fn convert_from_base(numbers: &[u32], from_base: u32) -> Result<u32, Error> {
    require(from_base >= 2, Error::InvalidInputBase)?;

    let mut number = 0u32;
    for &digit in numbers {
        require(digit < from_base, Error::InvalidDigit(digit))?;

        number = number * from_base + digit;
    }

    Ok(number)
}

fn convert_to_base(mut number: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    require(to_base >= 2, Error::InvalidOutputBase)?;

    if number == 0 {
        return Ok(vec![0]);
    }

    let mut ret = vec![];
    while number > 0 {
        ret.push(number % to_base);
        number /= to_base;
    }

    ret.reverse();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_bit_one_to_decimal() {
        let input_base = 2;
        let input_digits = &[1];
        let output_base = 10;
        let output_digits = vec![1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn binary_to_single_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1];
        let output_base = 10;
        let output_digits = vec![5];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn single_decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[5];
        let output_base = 2;
        let output_digits = vec![1, 0, 1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn binary_to_multiple_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[4, 2];
        let output_base = 2;
        let output_digits = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn trinary_to_hexadecimal() {
        let input_base = 3;
        let input_digits = &[1, 1, 2, 0];
        let output_base = 16;
        let output_digits = vec![2, 10];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn hexadecimal_to_trinary() {
        let input_base = 16;
        let input_digits = &[2, 10];
        let output_base = 3;
        let output_digits = vec![1, 1, 2, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn test_15_bit_integer() {
        let input_base = 97;
        let input_digits = &[3, 46, 60];
        let output_base = 73;
        let output_digits = vec![6, 10, 45];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn empty_list() {
        let input_base = 2;
        let input_digits = &[];
        let output_base = 10;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn single_zero() {
        let input_base = 10;
        let input_digits = &[0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn multiple_zeros() {
        let input_base = 10;
        let input_digits = &[0, 0, 0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn leading_zeros() {
        let input_base = 7;
        let input_digits = &[0, 6, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    #[ignore]
    fn input_base_is_one() {
        let input_base = 1;
        let input_digits = &[0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }
    #[test]
    #[ignore]
    fn input_base_is_zero() {
        let input_base = 0;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }
    #[test]
    #[ignore]
    fn invalid_positive_digit() {
        let input_base = 2;
        let input_digits = &[1, 2, 1, 0, 1, 0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidDigit(2))
        );
    }
    #[test]
    #[ignore]
    fn output_base_is_one() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 1;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
    #[test]
    #[ignore]
    fn output_base_is_zero() {
        let input_base = 10;
        let input_digits = &[7];
        let output_base = 0;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
}
