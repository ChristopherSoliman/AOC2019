use std::collections::{HashMap, HashSet};

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const A_UPPER: u32 = 'A' as u32;
const A_LOWER: u32 = 'a' as u32;

fn is_key_found(door: &char, bit_map: &u32) -> bool {
    bit_map & (1 << (*door as u32 - A_UPPER)) != 0
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

    let found_keys = 2_u32.pow(keys_count) - 1;
    const NEW_VEC: Vec<char> = vec![];
    let mut keys: [Vec<char>; 4] = [NEW_VEC; 4];
    let mut doors: [Vec<char>; 4] = [NEW_VEC; 4];

    for i in 0..height as usize {
        for j in 0..width as usize {
            if ('a'..='z').contains(&grid[i][j]) {
                let mut k = 3;
                if i < height as usize / 2 && j < width as usize / 2 {
                    k = 0;
                } else if i < height as usize / 2 && j > width as usize / 2 {
                    k = 1;
                } else if i > height as usize / 2 && j < width as usize / 2 {
                    k = 2;
                }
                keys[k].push(grid[i][j]);
            }
            if ('A'..='Z').contains(&grid[i][j]) {
                let mut k = 3;
                if i < height as usize / 2 && j < width as usize / 2 {
                    k = 0;
                } else if i < height as usize / 2 && j > width as usize / 2 {
                    k = 1;
                } else if i > height as usize / 2 && j > width as usize / 2 {
                    k = 2;
                }
                doors[k].push(grid[i][j]);
            }
        }
    }

    let mut masks: [u32; 4] = [0; 4];
    for i in 0..doors.len() {
        for door in &doors[i] {
            masks[i] |= 1 << (*door as u32 - A_UPPER);
        }
    }

    let start_1 = (start.0 - 1, start.1 - 1);
    let start_2 = (start.0 - 1, start.1 + 1);
    let start_3 = (start.0 + 1, start.1 + 1);
    let start_4 = (start.0 + 1, start.1 - 1);

    grid[start.0 - 1][start.1] = '#';
    grid[start.0 + 1][start.1] = '#';
    grid[start.0][start.1 - 1] = '#';
    grid[start.0][start.1 + 1] = '#';

    let mut q: Vec<([(usize, usize); 4], u32, u32)> =
        vec![([start_1, start_2, start_3, start_4], 0, 0)];
    let mut seen: HashSet<([(usize, usize); 4], u32)> = HashSet::new();
    let mut cache: HashMap<((usize, usize), usize, u32), Vec<((usize, usize), u32, u32)>> =
        HashMap::new();

    while !q.is_empty() {
        let (robots, keys, steps) = q.remove(0);
        if seen.len() % 1000 == 0 {
            println!("{}, {:0b}", seen.len(), keys);
        }
        if keys == found_keys {
            return steps;
        }
        if seen.contains(&(robots, keys)) {
            continue;
        }
        seen.insert((robots, keys));

        for i in 0..4 {
            let mut new_robots = robots.clone();
            let options = possible_moves(&robots[i], &grid, &(keys & masks[i]), &i, &mut cache);
            for (new_pos, new_keys, new_steps) in options {
                new_robots[i] = new_pos;
                q.push((new_robots, new_keys | keys, new_steps + steps));
            }
            //for dir in DIRS {
            //    let (nr, nc) = (
            //        robots[i].0 as i32 + dir.0 as i32,
            //        robots[i].1 as i32 + dir.1 as i32,
            //    );
            //    if nr < 0 || nr >= height || nc < 0 || nc >= width {
            //        continue;
            //    }
            //    let (nr, nc) = (nr as usize, nc as usize);
            //    let mut new_keys = keys;
            //    match grid[nr][nc] {
            //        '#' => continue,
            //        'A'..='Z' => {
            //            if !is_key_found(&grid[nr][nc], &keys) {
            //                continue;
            //            }
            //        }
            //        'a'..='z' => new_keys = add_key(&grid[nr][nc], &mut new_keys),
            //        _ => {}
            //    }
            //    new_robots[i] = (nr, nc);
            //    q.push((new_robots, new_keys, steps + 1));
            //}
        }
        q.sort_by_key(|v| v.2);
    }
    panic!("no solution found");
}

fn possible_moves(
    pos: &(usize, usize),
    grid: &Vec<Vec<char>>,
    original_keys: &u32,
    robot: &usize,
    cache: &mut HashMap<((usize, usize), usize, u32), Vec<((usize, usize), u32, u32)>>, //original_seen: &HashSet<((usize, usize), u32)>,
) -> Vec<((usize, usize), u32, u32)> {
    if let Some(result) = cache.get(&(*pos, *robot, *original_keys)) {
        return result.to_vec();
    }
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut q: Vec<((usize, usize), u32, u32)> = vec![(*pos, *original_keys, 0)];
    let mut seen: HashSet<((usize, usize), u32)> = HashSet::new();

    let mut options: HashMap<((usize, usize), u32), u32> = HashMap::new();
    while !q.is_empty() {
        let (robot, keys, dist) = q.remove(0);
        if seen.contains(&(robot, keys)) {
            //|| original_seen.contains(&(robot, keys)) {
            continue;
        }
        seen.insert((robot, keys));
        for dir in DIRS {
            let (nr, nc) = (robot.0 as i32 + dir.0 as i32, robot.1 as i32 + dir.1 as i32);
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let mut new_keys = keys;
            match grid[nr][nc] {
                '#' => continue,
                'A'..='Z' => {
                    let new_robot = (nr, nc);
                    if !is_key_found(&grid[nr][nc], &new_keys) {
                        let old_dist = options.get(&(robot, new_keys)).unwrap_or(&u32::MAX);
                        if *old_dist > dist {
                            options.insert((robot, new_keys), dist);
                        }
                        continue;
                    }
                    q.push((new_robot, new_keys, dist + 1));
                }
                'a'..='z' => {
                    let new_robot = (nr, nc);
                    new_keys = add_key(&grid[nr][nc], &mut new_keys);
                    q.push((new_robot, new_keys, dist + 1));
                    let old_dist = options.get(&(new_robot, new_keys)).unwrap_or(&u32::MAX);
                    if *old_dist > dist + 1 {
                        options.insert((new_robot, new_keys), dist + 1);
                    }
                }
                _ => {
                    let new_robot = (nr, nc);
                    q.push((new_robot, new_keys, dist + 1));
                }
            }
        }
    }
    //println!("{:?}", options);
    let options = options
        .into_iter()
        .map(|(k, v)| (k.0, k.1, v))
        .collect::<Vec<_>>();

    cache.insert((*pos, *robot, *original_keys), options.clone());

    options
}
