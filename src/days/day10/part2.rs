use std::{collections::HashMap, hash::Hash};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Angle {
    dc: i32,
    dr: i32,
}

impl Angle {
    pub fn cmp(&self, other: &Angle) -> std::cmp::Ordering {
        self.get_angle().partial_cmp(&other.get_angle()).unwrap()
    }

    fn get_angle(&self) -> f64 {
        let mut angle;
        angle = (-self.dr as f64).atan2(self.dc as f64);
        angle = std::f64::consts::FRAC_PI_2 - angle;
        angle.rem_euclid(std::f64::consts::PI * 2.0)
    }
}

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, char)| {
                    if char == '#' {
                        return Some((row, col));
                    }
                    None
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let opt = (19, 20); // from part1

    let mut m: HashMap<Angle, Vec<(u32, (usize, usize))>> = HashMap::new();
    let mut angles: Vec<Angle> = Vec::new();
    for asteroid in asteroids {
        let (mult, simple) = simplify(&(
            asteroid.0 as i32 - opt.0 as i32,
            asteroid.1 as i32 - opt.1 as i32,
        ));
        if simple.0 == 0 && simple.1 == 0 {
            continue;
        }
        let angle = Angle {
            dr: simple.0,
            dc: simple.1,
        };

        m.entry(angle)
            .and_modify(|v| {
                v.push((mult, asteroid));
                v.sort_by_key(|b| b.0);
            })
            .or_insert(vec![(mult, asteroid)]);

        if !angles.contains(&angle) {
            angles.push(angle);
        }
    }

    angles.sort_by(|a, b| a.cmp(&b));

    let mut vaporized = 0;
    let mut i = 0;
    let length = angles.len();
    let mut last: (usize, usize) = (0, 0);
    loop {
        if vaporized == 200 {
            break;
        }
        let angle = angles[i % length];
        let asts = m.get_mut(&angle).unwrap();
        if asts.len() > 0 {
            last = asts.remove(0).1;
            vaporized += 1;
        }

        i += 1;
    }

    (last.1 * 100 + last.0) as u32
}

fn simplify(slope: &(i32, i32)) -> (u32, (i32, i32)) {
    let simp = slope.clone();
    let mut gcd: u32 = 1;
    let mut max_gcd: u32 = 1;

    while gcd <= std::cmp::max(slope.0.abs(), slope.1.abs()) as u32 {
        if slope.0.rem_euclid(gcd as i32) == 0 && slope.1.rem_euclid(gcd as i32) == 0 {
            max_gcd = gcd;
        }
        gcd += 1
    }
    (max_gcd, (simp.0 / max_gcd as i32, simp.1 / max_gcd as i32))
}
