pub fn part1() -> u32 {
    let mut lb = 240920;
    let ub = 789857;

    let mut sum = 0;

    while lb < ub {
        let mut adjacent = false;
        let mut increasing = true;
        let mut val = lb.clone();
        let mut prev = -1;
        while val > 0 {
            let digit = val % 10;
            if prev != -1 {
                if digit > prev {
                    increasing = false;
                    break;
                } else if prev == digit {
                    adjacent = true;
                }
            }
            if !increasing {
                break;
            }
            prev = digit;
            val /= 10;
        }
        if adjacent && increasing {
            sum += 1;
        }
        lb += 1;
    }
    sum
}
