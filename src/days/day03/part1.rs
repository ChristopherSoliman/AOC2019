pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let (in1, in2) = input.split_once("\r\n").unwrap();

    let mut start = (0, 0);

    let wire_1 = in1
        .trim()
        .split(",")
        .map(|m| {
            let mut m = m.to_string();
            let dir = m.remove(0);
            let new_pos: ((i32, i32), (i32, i32));
            match dir {
                'R' => {
                    new_pos = (start, (start.0 + m.parse::<i32>().unwrap(), start.1));
                    start = new_pos.1;
                }
                'L' => {
                    new_pos = (start, (start.0 - m.parse::<i32>().unwrap(), start.1));
                    start = new_pos.1;
                }
                'U' => {
                    new_pos = (start, (start.0, start.1 + m.parse::<i32>().unwrap()));
                    start = new_pos.1;
                }
                'D' => {
                    new_pos = (start, (start.0, start.1 - m.parse::<i32>().unwrap()));
                    start = new_pos.1;
                }
                _ => unreachable!("Invalid direction"),
            }
            new_pos
        })
        .collect::<Vec<_>>();

    start = (0, 0);

    let wire_2 = in2
        .trim()
        .split(",")
        .map(|m| {
            let mut m = m.to_string();
            let dir = m.remove(0);
            let new_pos: ((i32, i32), (i32, i32));
            match dir {
                'R' => {
                    new_pos = (start, (start.0 + m.parse::<i32>().unwrap(), start.1));
                    start = new_pos.1;
                }
                'L' => {
                    new_pos = (start, (start.0 - m.parse::<i32>().unwrap(), start.1));
                    start = new_pos.1;
                }
                'U' => {
                    new_pos = (start, (start.0, start.1 + m.parse::<i32>().unwrap()));
                    start = new_pos.1;
                }
                'D' => {
                    new_pos = (start, (start.0, start.1 - m.parse::<i32>().unwrap()));
                    start = new_pos.1;
                }
                _ => unreachable!("Invalid direction"),
            }
            new_pos
        })
        .collect::<Vec<_>>();

    let mut intersections: Vec<(i32, i32)> = vec![];
    for path1 in &wire_1 {
        for path2 in &wire_2 {
            if let Some(intersection) = intersect(path1, path2) {
                intersections.push(intersection);
            }
        }
    }

    intersections
        .iter()
        .map(|p| (p.0.abs() + p.1.abs()) as u32)
        .min()
        .unwrap()
}

fn intersect(a: &((i32, i32), (i32, i32)), b: &((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    let (a_start, a_end) = a;
    let (b_start, b_end) = b;

    let diff_a = (a_end.0 - a_start.0, a_end.1 - a_start.1);
    let diff_b = (b_end.0 - b_start.0, b_end.1 - b_start.1);

    if diff_a.0 != 0 && diff_b.0 != 0 {
        return None;
    }

    if diff_a.0 != 0 {
        if (a_start.0..a_end.0).contains(&b_start.0) {
            if (b_end.1..b_start.1).contains(&a_start.1) {
                return Some((b_start.0, a_start.1));
            }
        }
    } else {
        if (b_start.0..b_end.0).contains(&a_start.0) {
            if (a_end.1..a_start.1).contains(&b_start.1) {
                return Some((a_start.0, b_start.1));
            }
        }
    }
    None
}
