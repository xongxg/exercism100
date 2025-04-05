#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..8, 0..8) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || (self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queen_with_a_valid_position() {
        let chess_position = ChessPosition::new(2, 2);
        assert!(chess_position.is_some());
    }
    #[test]
    #[ignore]
    fn queen_must_have_positive_row() {
        let chess_position = ChessPosition::new(-2, 2);
        assert!(chess_position.is_none());
    }
    #[test]
    #[ignore]
    fn queen_must_have_row_on_board() {
        let chess_position = ChessPosition::new(8, 4);
        assert!(chess_position.is_none());
    }
    #[test]
    #[ignore]
    fn queen_must_have_positive_column() {
        let chess_position = ChessPosition::new(2, -2);
        assert!(chess_position.is_none());
    }
    #[test]
    #[ignore]
    fn queen_must_have_column_on_board() {
        let chess_position = ChessPosition::new(4, 8);
        assert!(chess_position.is_none());
    }
    #[test]
    #[ignore]
    fn cannot_attack() {
        let mut white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
        assert!(!white_queen.can_attack(&black_queen));
    }
    #[test]
    #[ignore]
    fn can_attack_on_same_row() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }
    #[test]
    #[ignore]
    fn can_attack_on_same_column() {
        let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }
    #[test]
    #[ignore]
    fn can_attack_on_first_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }
    #[test]
    #[ignore]
    fn can_attack_on_second_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }
    #[test]
    #[ignore]
    fn can_attack_on_third_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }
}
