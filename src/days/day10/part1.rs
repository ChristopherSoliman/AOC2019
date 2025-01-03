use std::collections::HashSet;

pub fn part1(path: &str) -> u32 {
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

    let diffs = asteroids
        .iter()
        .map(|ast| {
            asteroids
                .iter()
                .map(|a| simplify(&(a.0 as i32 - ast.0 as i32, a.1 as i32 - ast.1 as i32)))
                .collect::<HashSet<_>>()
        })
        .max_by_key(|x| x.len())
        .unwrap();
    diffs.len() as u32 - 1
}

fn simplify(slope: &(i32, i32)) -> (i32, i32) {
    let simp = slope.clone();
    let mut gcd = 1;
    let mut max_gcd = 1;

    while gcd <= std::cmp::max(slope.0.abs(), slope.1.abs()) {
        if slope.0.rem_euclid(gcd) == 0 && slope.1.rem_euclid(gcd) == 0 {
            max_gcd = gcd;
        }
        gcd += 1
    }
    (simp.0 / max_gcd, simp.1 / max_gcd)
}
