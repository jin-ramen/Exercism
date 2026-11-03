pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = (student.chars().next().unwrap() as usize - 65) * 2;
    
    diagram
        .lines()
        .flat_map(|line| {
            line.chars()
                .skip(index)
                .take(2)
        })
        .map(|p| match p {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!("plant not found"),
        })
        .collect()
}
