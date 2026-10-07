pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)|
                    if ch != "*" {
                        get_adjacent_flower_count(r, c, garden);
                    }
                )
                .collect()
        })
        .collect()
}

fn get_adjacent_flower_count(r: usize, c: usize, gardem: &[&str]) -> u32 {
    todo!("count number of adjacent flowers and return. if none returns 0")
}