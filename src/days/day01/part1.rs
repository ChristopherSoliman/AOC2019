pub fn part1(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .map(|l| {
            let m = l.parse::<u32>().expect("Failed to parse int");
            m / 3 - 2
        })
        .sum::<u32>()
}
