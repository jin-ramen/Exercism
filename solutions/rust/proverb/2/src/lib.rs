pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|window| { // yields None if window[0] = window[1] = None
            format!("For want of a {} the {} was lost.\n", window[0], window[1])
        })
        .chain(
            list.first() // yields None if None
                .map(|s| format!("And all for the want of a {s}."))
        )
        .collect::<String>() // collect into empty String if None
}
