use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .map(|b| {
            let mut res = vec![];
            if *b != 0 {
                let mut n = 1;
                while b * n < limit {
                    res.push(b * n);
                    n += 1;
                }
            }
            res
        })
        .flatten()
        .collect::<HashSet<u32>>()
        .into_iter()
        .sum()
}
