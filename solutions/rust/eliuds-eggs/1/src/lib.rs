pub fn egg_count(mut display_value: u32) -> usize {
    let mut binary_digits = vec![];
    while display_value > 0 {
        binary_digits.push(display_value % 2);
        display_value /= 2;
    }

    binary_digits
        .iter()
        .filter(|d| **d == 1)
        .count()
}
