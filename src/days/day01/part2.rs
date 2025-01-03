pub fn part2(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .map(|l| get_fuel(l.parse::<u32>().expect("Failed to parse int")))
        .sum::<u32>()
}

fn get_fuel(weight: u32) -> u32 {
    let mut required = weight / 3;
    if required <= 2 {
        return 0;
    }
    required -= 2;
    return required + get_fuel(required);
}
