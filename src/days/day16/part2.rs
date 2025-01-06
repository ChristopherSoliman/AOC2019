pub fn part2(path: &str) -> u32 {
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

    signal = signal.repeat(10000);

    let mut phase = 0;
    let mut offset = 0;
    for i in 0..7 {
        offset += signal[i] * 10_u32.pow(6 - i as u32);
    }
    signal = signal[offset as usize..signal.len()].to_vec();

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
    let mut new_signal: Vec<u32> = signal.to_vec();

    for i in (0..signal.len()).rev() {
        if i == signal.len() - 1 {
            new_signal[i] = signal[i] % 10;
        } else {
            new_signal[i] = (signal[i] + new_signal[i + 1]) % 10;
        }
    }
    new_signal
}
