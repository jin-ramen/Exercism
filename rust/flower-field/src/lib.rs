pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)|
                    match ch {
                        '*' => '*',
                        _ => match get_flower_count(r, c, garden) {
                            0 => ' ',
                            n => char::from(n as u8 + b'0'), // char implements the From trait
                        },
                    }
                )
                .collect()
        })
        .collect()
}

fn get_flower_count(r: usize, c: usize, garden: &[&str]) -> usize {
    garden[r.saturating_sub(1)..(r+2).min(garden.len())]
        .iter() // [{"", *, ""}, {"", n, ""}, {*, *, ""}]
        .flat_map(|row| row[c.saturating_sub(1)..(c+2).min(row.len())].bytes()) // flatten -> ["", *, "", "", n, "", *, *, ""]
        .filter(|byte| byte==&b'*') // [*, *, *]
        .count()
}