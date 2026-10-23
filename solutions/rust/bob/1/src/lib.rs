pub fn reply(message: &str) -> &str {
    let msg: String = message.chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let is_question = msg.ends_with('?');

    let is_all_caps = msg.chars().any(|c| c.is_alphabetic())
        && msg.chars()
            .filter(|c| c.is_alphabetic())   
            .all(|c| c.is_uppercase());

    let is_whitespace = msg.is_empty();

    match (is_question, is_all_caps, is_whitespace) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, false, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
