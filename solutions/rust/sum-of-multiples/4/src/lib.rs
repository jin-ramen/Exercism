use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|b| get_multiples(limit, *b))
        .collect::<HashSet<u32>>()
        .into_iter()
        .sum()
}

fn get_multiples(limit: u32, base: u32) -> Vec<u32> {
    let mut res = vec![];
    if base != 0 {
        let mut n = 1;
        while base * n < limit {
            res.push(base * n);
            n += 1;
        }
    }
    res
}