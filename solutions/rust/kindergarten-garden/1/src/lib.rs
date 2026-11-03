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
            'G' | 'g' => "grass",
            'C' | 'c' => "clover",
            'R' | 'r' => "radishes",
            'V' | 'v' => "violet",
            _ => panic!("plant not found"),
        })
        .collect()
}
