#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    SuperList,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let super_list = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|x| x == second_list);

    let sub_list = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|x| x == first_list);

    match (super_list, sub_list) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::SuperList,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

#[cfg(test)]
mod tests {
    use super::sublist;
    use super::Comparison;

    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::SuperList;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn false_start() {
        let list_one: &[i32] = &[1, 2, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn consecutive() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn sublist_at_start() {
        let list_one: &[i32] = &[0, 1, 2];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn sublist_in_middle() {
        let list_one: &[i32] = &[2, 3, 4];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn sublist_at_end() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn at_start_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::SuperList;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn in_middle_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::SuperList;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn at_end_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::SuperList;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn first_list_missing_element_from_second_list() {
        let list_one: &[i32] = &[1, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn second_list_missing_element_from_first_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn first_list_missing_additional_digits_from_second_list() {
        let list_one: &[i32] = &[1, 2];
        let list_two: &[i32] = &[1, 22];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn order_matters_to_a_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[3, 2, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn same_digits_but_different_numbers() {
        let list_one: &[i32] = &[1, 0, 1];
        let list_two: &[i32] = &[10, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}
