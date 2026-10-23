pub fn reply(message: &str) -> &str {
    let question = message.trim().ends_with('?');

    let shouting = message.chars().any(|c| c.is_alphabetic())
        && message.to_uppercase() == message;

    let silence = message.trim().is_empty();

    match (question, shouting, silence) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (false, false, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
