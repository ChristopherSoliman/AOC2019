pub fn part2() -> u32 {
    let mut lb = 240920;
    let ub = 789857;
    let mut sum = 0;

    while lb < ub {
        let mut adjacent = false;
        let mut increasing = true;
        let mut val = lb.clone();
        let mut prev = -1;
        let mut seen = [0; 10];
        while val > 0 {
            let digit = val % 10;
            if prev != -1 {
                if digit > prev {
                    increasing = false;
                    break;
                }
                if digit == prev {
                    seen[digit as usize] += 1;
                }
            }
            if !increasing {
                break;
            }
            prev = digit;
            val /= 10;
        }
        for d in seen {
            if d == 1 {
                adjacent = true;
            }
        }
        if adjacent && increasing {
            sum += 1;
        }
        lb += 1;
    }
    sum
}
