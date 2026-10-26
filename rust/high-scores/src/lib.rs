#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores
            .iter()
            .map(Some)
            .fold([None; 3], |[a, b, c], x| {
                if x <= c { [a, b, c] }
                else if x <= b { [a, b, x] }
                else if x <= a { [a, x, b] }
                else { [x, a, b] }
            })
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u32>>()
    }
}
