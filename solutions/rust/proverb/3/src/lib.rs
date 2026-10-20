pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
        .chain(list.first().map(|s| format!("And all for the want of a {s}.")))
        .collect()
}
