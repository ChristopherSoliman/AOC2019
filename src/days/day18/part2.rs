use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const A_UPPER: u32 = 'A' as u32;
const A_LOWER: u32 = 'a' as u32;

#[derive(Eq, PartialEq)]
struct State {
    robots: [(usize, usize); 4],
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

fn can_unlock_door(door: &char, bit_map: &u32) -> bool {
    bit_map & (1 << (*door as u32 - A_UPPER)) != 0
}

fn is_key_found(key: &char, bit_map: &u32) -> bool {
    bit_map & (1 << (*key as u32 - A_LOWER)) != 0
}

fn add_key(key: &char, bit_map: &u32) -> u32 {
    *bit_map | 1 << (*key as u32 - A_LOWER)
}

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut start = (0, 0);
    let mut keys_count = 0;

    let mut grid = input
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
                            return '#';
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

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let all_keys = 2_u32.pow(keys_count) - 1;
    const NEW_VEC: Vec<char> = vec![];
    let mut keys: [Vec<char>; 4] = [NEW_VEC; 4];

    for i in 0..height as usize {
        for j in 0..width as usize {
            if ('a'..='z').contains(&grid[i][j]) {
                let mut k = 3;
                if i < height as usize / 2 && j < width as usize / 2 {
                    k = 0;
                } else if i < height as usize / 2 && j > width as usize / 2 {
                    k = 1;
                } else if i > height as usize / 2 && j > width as usize / 2 {
                    k = 2;
                }
                keys[k].push(grid[i][j]);
            }
        }
    }

    let mut key_masks: [u32; 4] = [0; 4];
    for i in 0..keys.len() {
        for key in &keys[i] {
            key_masks[i] |= 1 << (*key as u32 - A_LOWER);
        }
    }
    let starts: [(usize, usize); 4] = [
        (start.0 - 1, start.1 - 1),
        (start.0 - 1, start.1 + 1),
        (start.0 + 1, start.1 + 1),
        (start.0 + 1, start.1 - 1),
    ];

    grid[start.0 - 1][start.1] = '#';
    grid[start.0 + 1][start.1] = '#';
    grid[start.0][start.1 - 1] = '#';
    grid[start.0][start.1 + 1] = '#';

    let mut q: BinaryHeap<State> = BinaryHeap::new();
    let mut seen: HashSet<([(usize, usize); 4], u32)> = HashSet::new();
    let mut cache: HashMap<((usize, usize), u32), Vec<((usize, usize), u32, char)>> =
        HashMap::new();

    q.push(State {
        robots: starts,
        keys: 0,
        steps: 0,
    });

    while let Some(state) = q.pop() {
        if state.keys == all_keys {
            return state.steps;
        }
        if seen.contains(&(state.robots, state.keys)) {
            continue;
        }
        seen.insert((state.robots, state.keys));

        for (i, robot) in state.robots.iter().enumerate() {
            for (npos, nd, key) in reachable_keys(&robot, &state.keys, &grid, &mut cache).iter() {
                let mut new_robots = state.robots.clone();
                new_robots[i] = *npos;
                let new_keys = add_key(key, &state.keys);
                q.push(State {
                    robots: new_robots,
                    keys: new_keys,
                    steps: state.steps + nd,
                });
            }
        }
    }

    panic!("No solution found");
}

fn reachable_keys(
    pos: &(usize, usize),
    keys: &u32,
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<((usize, usize), u32), Vec<((usize, usize), u32, char)>>,
) -> Vec<((usize, usize), u32, char)> {
    if let Some(v) = cache.get(&(*pos, *keys)) {
        return v.clone();
    }
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut reachable: Vec<((usize, usize), u32, char)> = vec![];
    let mut q: VecDeque<((usize, usize), u32)> = VecDeque::from(vec![(*pos, 0)]);
    let mut seen: HashSet<((usize, usize), u32)> = HashSet::new();

    while let Some((pos, d)) = q.pop_front() {
        if ('a'..='z').contains(&grid[pos.0][pos.1]) && !is_key_found(&grid[pos.0][pos.1], &keys) {
            reachable.push((pos, d, grid[pos.0][pos.1]));
        }
        for dir in DIRS {
            let (nr, nc) = (pos.0 as i32 + dir.0 as i32, pos.1 as i32 + dir.1 as i32);
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let new_robot = (nr, nc);
            if seen.contains(&(new_robot, *keys)) {
                continue;
            }
            seen.insert((new_robot, *keys));
            match grid[nr][nc] {
                '#' => continue,
                'A'..='Z' if !can_unlock_door(&grid[nr][nc], &keys) => continue,
                'a'..='z' => {}
                _ => {}
            }
            q.push_back((new_robot, d + 1));
        }
    }
    cache.insert((*pos, *keys), reachable.clone());

    reachable
}
