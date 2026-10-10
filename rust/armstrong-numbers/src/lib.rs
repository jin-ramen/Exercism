pub fn is_armstrong_number(num: u32) -> bool {
    let power = if num == 0 { 1 } else { num.ilog10() + 1 };

    let n: u32 = num
        .to_string()
        .bytes()
        .map(|b| ((b - b'0') as u32).pow(power))
        .sum();

    n == num
}
