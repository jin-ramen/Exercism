pub fn raindrops(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0, n % 7 == 0) {
        (false, false, false) => n.to_string(),
        (three, five, seven) => format!(
            "{}{}{}",
            if three { "Pling" } else { "" },
            if five  { "Plang" } else { "" },
            if seven { "Plong" } else { "" }
        ),
    }
}