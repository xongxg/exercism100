use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(&self) -> HashSet<(u64, u64)> {
        self.factors.clone()
    }
}

pub fn is_palindrome(x: u64) -> bool {
    let y = x.to_string();
    y.chars().eq(y.chars().rev())
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut small_palindrome = Option::<Palindrome>::None;
    let mut big_palindrome = Option::<Palindrome>::None;

    for i in min..=max {
        for j in i..=max {
            let n = i * j;

            if !is_palindrome(n) {
                continue;
            }

            match small_palindrome {
                Some(ref mut sp) if sp.value > n => {
                    sp.value = n;
                    sp.factors = HashSet::from([(i, j)]);
                }
                Some(ref mut sp) if sp.value == n => {
                    sp.factors.insert((i, j));
                }
                None => {
                    small_palindrome = Some(Palindrome {
                        value: n,
                        factors: HashSet::from([(i, j)]),
                    });
                }
                _ => {}
            }

            match big_palindrome {
                Some(ref mut pb) if pb.value < n => {
                    pb.value = n;
                    pb.factors = HashSet::from([(i, j)]);
                }
                Some(ref mut pb) if pb.value == n => {
                    pb.factors.insert((i, j));
                }
                None => {
                    big_palindrome = Some(Palindrome {
                        value: n,
                        factors: HashSet::from([(i, j)]),
                    });
                }
                _ => {}
            }
        }
    }

    match (small_palindrome, big_palindrome) {
        (Some(small_palindrome), Some(big_palindrome)) => Some((small_palindrome, big_palindrome)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_the_smallest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 1)]));
    }
    #[test]
    #[ignore]
    fn find_the_largest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 9), (3, 3)]));
    }
    #[test]
    #[ignore]
    fn find_the_smallest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 121);
        assert_eq!(pal.into_factors(), HashSet::from([(11, 11)]));
    }
    #[test]
    #[ignore]
    fn find_the_largest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9009);
        assert_eq!(pal.into_factors(), HashSet::from([(91, 99)]));
    }
    #[test]
    #[ignore]
    fn find_the_smallest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10201);
        assert_eq!(pal.into_factors(), HashSet::from([(101, 101)]));
    }
    #[test]
    #[ignore]
    fn find_the_largest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 906609);
        assert_eq!(pal.into_factors(), HashSet::from([(913, 993)]));
    }
    #[test]
    #[ignore]
    fn find_the_smallest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1002001);
        assert_eq!(pal.into_factors(), HashSet::from([(1001, 1001)]));
    }
    #[test]
    #[ignore]
    fn find_the_largest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 99000099);
        assert_eq!(pal.into_factors(), HashSet::from([(9901, 9999)]));
    }
    #[test]
    #[ignore]
    fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(1002, 1003);
        assert!(output.is_none());
    }
    #[test]
    #[ignore]
    fn empty_result_for_largest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(15, 15);
        assert!(output.is_none());
    }
    #[test]
    #[ignore]
    fn error_result_for_smallest_if_min_is_more_than_max() {
        let output = palindrome_products(10000, 1);
        assert!(output.is_none());
    }
    #[test]
    #[ignore]
    fn error_result_for_largest_if_min_is_more_than_max() {
        let output = palindrome_products(2, 1);
        assert!(output.is_none());
    }
    #[test]
    #[ignore]
    fn smallest_product_does_not_use_the_smallest_factor() {
        let output = palindrome_products(3215, 4000);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10988901);
        assert_eq!(pal.into_factors(), HashSet::from([(3297, 3333)]));
    }
}
