#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut positions: Vec<Point> = vec![];
    let mut velocities: Vec<Point> = vec![];

    for line in input.lines() {
        let split = line
            .strip_prefix("<")
            .unwrap()
            .strip_suffix(">")
            .unwrap()
            .split(",");
        let mut vals: Vec<i32> = vec![];
        for coord in split {
            let (_, val) = coord.split_once("=").unwrap();
            vals.push(val.parse::<i32>().expect("failed to parse int"));
        }

        positions.push(Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        });

        velocities.push(Point { x: 0, y: 0, z: 0 });
    }

    let mut time = 0;
    while time < 1000 {
        update_velocities(&positions, &mut velocities);

        positions = positions
            .iter()
            .zip(velocities.clone())
            .map(|(p, v)| Point {
                x: p.x + v.x,
                y: p.y + v.y,
                z: p.z + v.z,
            })
            .collect::<Vec<_>>();
        time += 1;
    }

    positions
        .iter()
        .zip(velocities)
        .map(|(p, v)| {
            let pot = p.x.abs() + p.y.abs() + p.z.abs();
            let kin = v.x.abs() + v.y.abs() + v.z.abs();
            (pot * kin) as u32
        })
        .sum::<u32>()
}

fn update_velocities(positions: &Vec<Point>, velocities: &mut Vec<Point>) {
    for i in 0..positions.len() {
        let mut dx = 0;
        let mut dy = 0;
        let mut dz = 0;
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            if positions[i].x != positions[j].x {
                dx += (positions[j].x - positions[i].x)
                    / (positions[i].x.abs_diff(positions[j].x)) as i32;
            }
            if positions[i].y != positions[j].y {
                dy += (positions[j].y - positions[i].y)
                    / (positions[i].y.abs_diff(positions[j].y)) as i32;
            }
            if positions[i].z != positions[j].z {
                dz += (positions[j].z - positions[i].z)
                    / (positions[i].z.abs_diff(positions[j].z)) as i32;
            }
        }
        velocities[i] = Point {
            x: velocities[i].x + dx,
            y: velocities[i].y + dy,
            z: velocities[i].z + dz,
        }
    }
}
