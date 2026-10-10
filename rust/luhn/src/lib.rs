/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut count = 0;
    let c: Option<u32> = code
        .replace(" ", "")
        .bytes()
        .rev()
        .enumerate()
        .map(|(i, b)| {
            if !b.is_ascii_digit() {
                return None;
            }
            count += 1;
            let n = (b - b'0') as u32;
            match i % 2 {
                0 => Some(n),
                _ => match n * 2 {
                    m if m > 9 => Some(m - 9),
                    m => Some(m),
                },
            }
        })
        .sum();
    
    c.map_or(false, |s| count > 1 && s % 10 == 0)
}
