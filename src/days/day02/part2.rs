pub fn part2(path: &str) -> u32 {
    let intcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<u32>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut intcode_copy = intcode.clone();
            if execute(&mut intcode_copy, noun as u32, verb as u32) == 19690720 {
                return (noun * 100 + verb) as u32;
            }
        }
    }
    panic!("No solution found");
}

fn execute(intcode: &mut Vec<u32>, noun: u32, verb: u32) -> u32 {
    intcode[1] = noun;
    intcode[2] = verb;

    let mut ipointer = 0;

    while intcode[ipointer] != 99 {
        let first_idx = intcode[ipointer + 1] as usize;
        let second_idx = intcode[ipointer + 2] as usize;
        let out_idx = intcode[ipointer + 3] as usize;
        match intcode[ipointer] {
            1 => intcode[out_idx] = intcode[first_idx] + intcode[second_idx],
            2 => intcode[out_idx] = intcode[first_idx] * intcode[second_idx],
            _ => unreachable!("Invalid instruction"),
        }

        ipointer += 4;
    }

    intcode[0]
}
