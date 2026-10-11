/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut count = 0;
    code.bytes()
        .filter(|b| b != &b' ')
        .rev()
        .map(|b| {
            if !b.is_ascii_digit() {
                return None;
            }
            count += 1;
            let n = (b - b'0') as u32;
            match count % 2 {
                1 => Some(n),
                _ => match n * 2 {
                    m if m > 9 => Some(m - 9),
                    m => Some(m),
                },
            }
        })
        .sum::<Option<u32>>()
        .is_some_and(|s| count > 1 && s % 10 == 0)
}
