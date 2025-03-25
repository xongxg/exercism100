#[derive(Debug)]
struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    fn scores(&self) -> &[u32] {
        self.scores
    }

    fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    fn personal_top_three(&self) -> Vec<u32> {
        let mut res_vec = self.scores.to_vec();

        res_vec.sort_by(|a, b| b.cmp(a));
        res_vec.truncate(3);

        res_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    #[ignore]
    fn latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    #[ignore]
    fn personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);
        assert_eq!(high_scores.personal_best(), Some(100));
    }
    #[test]
    #[ignore]
    fn personal_top_three_from_a_list_of_scores() {
        let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
    }
    #[test]
    #[ignore]
    fn personal_top_highest_to_lowest() {
        let high_scores = HighScores::new(&[20, 10, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
    }
    #[test]
    #[ignore]
    fn personal_top_when_there_is_a_tie() {
        let high_scores = HighScores::new(&[40, 20, 40, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
    }
    #[test]
    #[ignore]
    fn personal_top_when_there_are_less_than_3() {
        let high_scores = HighScores::new(&[30, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
    }
    #[test]
    #[ignore]
    fn personal_top_when_there_is_only_one() {
        let high_scores = HighScores::new(&[40]);
        assert_eq!(high_scores.personal_top_three(), vec![40]);
    }
    #[test]
    #[ignore]
    fn latest_score_unchanged_after_personal_top_scores() {
        let high_scores = HighScores::new(&[70, 50, 20, 30]);
        let _ = high_scores.personal_top_three();
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    #[ignore]
    fn scores_unchanged_after_personal_top_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        let _ = high_scores.personal_top_three();
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    #[ignore]
    fn latest_score_unchanged_after_personal_best() {
        let high_scores = HighScores::new(&[20, 70, 15, 25, 30]);
        let _ = high_scores.personal_best();
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    #[ignore]
    fn scores_unchanged_after_personal_best() {
        let expected = [20, 70, 15, 25, 30];
        let high_scores = HighScores::new(&expected);
        let _ = high_scores.personal_best();
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    #[ignore]
    fn latest_score_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.latest(), None);
    }
    #[test]
    #[ignore]
    fn personal_best_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.personal_best(), None);
    }
    #[test]
    #[ignore]
    fn personal_top_three_empty() {
        let high_scores = HighScores::new(&[]);
        assert!(high_scores.personal_top_three().is_empty());
    }
}
