pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = Vec::new();
        let row_count = self.0.clone() + 1;

        for row in 1..row_count {
            if row == 1 {
                result.push(vec![1]);
            } else {
                let last_row = result.last().unwrap().clone();
                result.push(get_next_row(last_row));
            }
        }

        result
    }
}

fn get_next_row(current_row: Vec<u32>) -> Vec<u32> {
    let mut next_row = Vec::new();

    next_row.push(1);
    for i in 1..current_row.len() {
        next_row.push(current_row[i - 1] + current_row[i]);
    }

    next_row.push(1);

    next_row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_rows() {
        let pt = PascalsTriangle::new(0);
        let expected: Vec<Vec<u32>> = vec![];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn single_row() {
        let pt = PascalsTriangle::new(1);
        let expected: Vec<Vec<u32>> = vec![vec![1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn two_rows() {
        let pt = PascalsTriangle::new(2);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn three_rows() {
        let pt = PascalsTriangle::new(3);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn four_rows() {
        let pt = PascalsTriangle::new(4);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn five_rows() {
        let pt = PascalsTriangle::new(5);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn six_rows() {
        let pt = PascalsTriangle::new(6);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    #[ignore]
    fn ten_rows() {
        let pt = PascalsTriangle::new(10);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
}
