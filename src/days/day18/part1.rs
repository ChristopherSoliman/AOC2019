use std::collections::HashSet;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut start = (0, 0);
    let mut keys_count = 0;

    let grid = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        '@' => {
                            start = (i, j);
                            return '.';
                        }
                        'a'..='z' => {
                            keys_count += 1;
                        }
                        _ => {}
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut q: Vec<((usize, usize), Vec<char>, u32)> = vec![(start, vec![], 0)];
    let mut seen: HashSet<((usize, usize), Vec<char>)> = HashSet::new();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    while !q.is_empty() {
        let (pos, keys, steps) = q.remove(0);
        if seen.contains(&(pos, keys.clone())) {
            continue;
        }
        if keys.len() == keys_count {
            return steps;
        }
        seen.insert((pos, keys.clone()));

        for dir in DIRS {
            let (nr, nc) = (pos.0 as i32 + dir.0 as i32, pos.1 as i32 + dir.1 as i32);
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let mut new_keys = keys.clone();
            match grid[nr][nc] {
                '#' => continue,
                'A'..='Z' => {
                    if !keys.contains(&grid[nr][nc].to_lowercase().next().unwrap()) {
                        continue;
                    }
                }
                'a'..='z' => {
                    if !new_keys.contains(&grid[nr][nc]) {
                        new_keys.push(grid[nr][nc]);
                        new_keys.sort();
                    }
                }
                _ => {}
            }
            q.push(((nr, nc), new_keys.clone(), steps + 1));
        }
    }
    panic!("no solution found");
}
