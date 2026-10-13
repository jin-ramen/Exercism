pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let res = (0..take_down)
        .map(|n| verse(start_bottles-n))
        .collect::<Vec<String>>()
        .join("\n\n");

    dbg!(&res);
    res
}

fn verse(num: u32) -> String {
    let first = match num {
        1 => format!("{} green bottle hanging on the wall,", num_to_word(num)),
        _ => format!("{} green bottles hanging on the wall,", num_to_word(num)),
    };

    let third = format!("And if one green bottle should accidentally fall,");

    let fourth = match num-1 {
        1 => format!("There'll be {} green bottle hanging on the wall.", num_to_word(num-1).to_lowercase()),
        _ => format!("There'll be {} green bottles hanging on the wall.", num_to_word(num-1).to_lowercase()),
    };

    format!("{}\n{}\n{}\n{}", first, first, third, fourth)
}

fn num_to_word(num: u32) -> String {
    match num {
        0 => String::from("no"),
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        _ => String::from("Overflow"),
    }
}
