use std::collections::{BinaryHeap, HashSet};

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const A_UPPER: u32 = 'A' as u32;
const A_LOWER: u32 = 'a' as u32;

#[derive(Eq, PartialEq)]
struct State {
    robot: (usize, usize),
    keys: u32,
    steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn is_key_found(door: &char, bit_map: &u32) -> bool {
    bit_map & (1 << (*door as u32 - A_UPPER)) != 0
}

fn add_key(key: &char, bit_map: &u32) -> u32 {
    *bit_map | 1 << (*key as u32 - A_LOWER)
}

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

    let all_keys = (1 << keys_count) - 1;
    let mut q: BinaryHeap<State> = BinaryHeap::new();
    let mut seen: HashSet<((usize, usize), u32)> = HashSet::new();
    q.push(State {
        robot: start,
        keys: 0,
        steps: 0,
    });

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    while let Some(state) = q.pop() {
        if state.keys == all_keys {
            return state.steps;
        }
        for dir in DIRS {
            let (nr, nc) = (
                state.robot.0 as i32 + dir.0 as i32,
                state.robot.1 as i32 + dir.1 as i32,
            );
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if seen.contains(&((nr, nc), state.keys)) {
                continue;
            }
            seen.insert(((nr, nc), state.keys));
            let mut new_keys = state.keys;
            match grid[nr][nc] {
                '#' => continue,
                'A'..='Z' if !is_key_found(&grid[nr][nc], &new_keys) => continue,
                'a'..='z' => new_keys = add_key(&grid[nr][nc], &new_keys),
                _ => {}
            }
            q.push(State {
                robot: (nr, nc),
                keys: new_keys,
                steps: state.steps + 1,
            });
        }
    }
    panic!("no solution found");
}
