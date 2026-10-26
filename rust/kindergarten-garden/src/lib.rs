pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = (student.chars().next().map(|c| c as u32)).unwrap() - 65;
    dbg!(index);
    diagram
        .split("\n")
        .map(|r| {
            r.chars()
                .enumerate()
                .filter(|(i, p)| *i as u32 == index || *i as u32 == index + 1)
                .map(|(_i, p)| p)
        })
        .collect::<Vec<char>>()
        .iter()
        .map(|p| match p {
            'G' => "Grass",
            'C' => "Clover",
            'R' => "Radish",
            'V' => "Violet",
            _ => panic!("plant not found"),
        })
        .collect()
}
