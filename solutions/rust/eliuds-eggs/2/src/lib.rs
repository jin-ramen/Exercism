pub fn egg_count(mut display_value: u32) -> usize {
    let mut res = 0;

    while display_value > 0 {
        res += display_value % 2;
        display_value /= 2;
    }

    res as usize
}
