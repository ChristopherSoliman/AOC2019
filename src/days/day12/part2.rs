#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut moons: Vec<[i32; 3]> = vec![];

    for line in input.lines() {
        let split = line
            .strip_prefix("<")
            .unwrap()
            .strip_suffix(">")
            .unwrap()
            .split(",");
        let mut vals: [i32; 3] = [0; 3];
        let mut i = 0;
        for coord in split {
            let (_, val) = coord.split_once("=").unwrap();
            vals[i] = val.parse::<i32>().expect("failed to parse int");
            i += 1;
        }

        moons.push(vals);
    }

    let first_pos = moons.clone();
    let x_vals = moons.iter().map(|v| v[0]).collect::<Vec<_>>();
    let y_vals = moons.iter().map(|v| v[1]).collect::<Vec<_>>();
    let z_vals = moons.iter().map(|v| v[2]).collect::<Vec<_>>();

    let loop_x = find_loop(&x_vals, &first_pos.iter().map(|v| v[0]).collect::<Vec<_>>());
    let loop_y = find_loop(&y_vals, &first_pos.iter().map(|v| v[1]).collect::<Vec<_>>());
    let loop_z = find_loop(&z_vals, &first_pos.iter().map(|v| v[2]).collect::<Vec<_>>());

    lcm(loop_x as u64, lcm(loop_y as u64, loop_z as u64))
}

fn find_loop(vals: &Vec<i32>, first_pos: &Vec<i32>) -> u32 {
    let mut vels: [i32; 4] = [0; 4];
    let mut vals = vals.clone();
    let mut time = 0;
    loop {
        let mut same = true;
        let old_vals = vals.clone();
        for i in 0..vals.len() {
            let new_val = get_position(&old_vals, &old_vals[i], &vels[i]);

            vals[i] = new_val;
            vels[i] = new_val - old_vals[i];

            if same && (first_pos[i] != vals[i] || vels[i] != 0) {
                same = false;
            }
        }

        time += 1;
        if same {
            break;
        }
    }
    time
}

fn get_position(others: &Vec<i32>, position: &i32, velocity: &i32) -> i32 {
    let mut new = position + velocity;
    for point in others {
        if point > position {
            new += 1;
        } else if point < position {
            new -= 1;
        }
    }
    new
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a.rem_euclid(b);
        a = temp;
    }

    a
}
