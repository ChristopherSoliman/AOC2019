pub fn part1(path: &str) -> u32 {
    let mut opcode = std::fs::read_to_string(path)
        .expect("File should be there")
        .lines()
        .flat_map(|l| {
            l.split(",")
                .map(|n| n.parse::<u32>().expect("Failed to parse int"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    opcode[1] = 12;
    opcode[2] = 2;

    let mut ipointer = 0;

    while opcode[ipointer] != 99 {
        let first_idx = opcode[ipointer + 1] as usize;
        let second_idx = opcode[ipointer + 2] as usize;
        let out_idx = opcode[ipointer + 3] as usize;
        match opcode[ipointer] {
            1 => opcode[out_idx] = opcode[first_idx] + opcode[second_idx],
            2 => opcode[out_idx] = opcode[first_idx] * opcode[second_idx],
            _ => unreachable!("Invalid instruction"),
        }

        ipointer += 4;
    }

    opcode[0]
}
