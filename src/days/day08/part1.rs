const IMAGE_SIZE: usize = 25 * 6;

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars = input.lines().flat_map(|l| l.chars());

    let mut max_zeros = u32::MAX;
    let mut zeros = 0;
    let mut ones = 0;
    let mut twos = 0;

    let mut sum = 0;

    chars.enumerate().for_each(|(i, c)| {
        if i % (IMAGE_SIZE) == 0 && i != 0 {
            if zeros < max_zeros {
                max_zeros = zeros;
                sum = ones * twos;
            }
            zeros = 0;
            ones = 0;
            twos = 0;
        }
        match c {
            '0' => zeros += 1,
            '1' => ones += 1,
            '2' => twos += 1,
            _ => {}
        }
    });

    sum
}
