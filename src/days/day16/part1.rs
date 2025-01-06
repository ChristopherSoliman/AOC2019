const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut signal = input
        .trim()
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut phase = 0;
    while phase < 100 {
        signal = apply_phase(&signal);
        phase += 1;
    }
    let mut sum = 0;
    for i in 0..8 {
        sum += signal[i] * 10_u32.pow(7 - i as u32);
    }
    sum
}

fn apply_phase(signal: &Vec<u32>) -> Vec<u32> {
    let mut new_signal: Vec<u32> = vec![];

    for i in 0..signal.len() {
        let mut new_val = 0;
        let mut k = 0;
        for j in i..signal.len() {
            if (j + 1) % (i + 1) == 0 {
                k += 1;
            }
            new_val += signal[j] as i32 * BASE_PATTERN[k % 4] as i32;
        }
        new_signal.push((new_val % 10).abs() as u32);
    }

    new_signal
}
