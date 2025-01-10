use std::collections::{HashMap, HashSet};

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let raw_grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid: Vec<Vec<bool>> = vec![];

    let height = raw_grid.len();
    let width = raw_grid.iter().map(|r| r.len()).max().unwrap();

    let (inner_i, inner_j) = get_inner_bounds(&raw_grid, &width, &height);

    let (outer_jumps, inner_jumps, start, end) =
        get_jumps(&raw_grid, &width, &height, &inner_i, &inner_j);

    for i in 0..height {
        let mut v = vec![];
        for j in 0..width {
            if j >= raw_grid[i].len() {
                v.push(false);
                continue;
            }
            v.push(raw_grid[i][j] == '.');
        }
        grid.push(v);
    }
    bfs(&grid, &start, &end, &outer_jumps, &inner_jumps)
}

fn bfs(
    grid: &Vec<Vec<bool>>,
    start: &(usize, usize),
    end: &(usize, usize),
    outer_jumps: &HashMap<(usize, usize), (usize, usize)>,
    inner_jumps: &HashMap<(usize, usize), (usize, usize)>,
) -> u32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut q: Vec<((usize, usize), u32, u32)> = vec![(*start, 0, 0)];
    let mut seen: HashSet<((usize, usize), u32)> = HashSet::new();

    while !q.is_empty() {
        let (pos, level, steps) = q.remove(0);
        if pos == *end && level == 0 {
            return steps;
        }
        if seen.contains(&(pos, level)) {
            continue;
        }
        seen.insert((pos, level));

        for dir in DIRS {
            let (nr, nc) = (pos.0 as i32 + dir.0 as i32, pos.1 as i32 + dir.1 as i32);
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if !grid[nr][nc] {
                continue;
            }
            q.push(((nr, nc), level, steps + 1));

            if level > 0 {
                if let Some(jump_pos) = outer_jumps.get(&(nr, nc)) {
                    if !seen.contains(&(*jump_pos, level - 1)) {
                        q.push((*jump_pos, level - 1, steps + 2));
                    }
                }
            }
            if let Some(jump_pos) = inner_jumps.get(&(nr, nc)) {
                if !seen.contains(&(*jump_pos, level + 1)) {
                    if level > 0 && seen.contains(&(*jump_pos, level - 1)) {
                        continue;
                    }
                    q.push((*jump_pos, level + 1, steps + 2));
                }
            }
        }
        q.sort_by_key(|v| v.1);
    }

    panic!("no solution found");
}

fn get_jumps(
    grid: &Vec<Vec<char>>,
    width: &usize,
    height: &usize,
    inner_i: &(usize, usize),
    inner_j: &(usize, usize),
) -> (
    HashMap<(usize, usize), (usize, usize)>,
    HashMap<(usize, usize), (usize, usize)>,
    (usize, usize),
    (usize, usize),
) {
    let mut labeled_pos: Vec<((usize, usize), String)> = vec![];
    let mut inner_jumps: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut outer_jumps: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for i in 2..height - 2 {
        for j in 2..width - 2 {
            let c = grid[i][j];
            if c == '.' {
                if i == 2 || (i == inner_i.1 && j > inner_j.0 && j < inner_j.1) {
                    labeled_pos.push(((i, j), format!("{}{}", grid[i - 2][j], grid[i - 1][j])));
                } else if i == height - 3 || (i == inner_i.0 && j > inner_j.0 && j < inner_j.1) {
                    labeled_pos.push(((i, j), format!("{}{}", grid[i + 1][j], grid[i + 2][j])));
                } else if j == 2 || (j == inner_j.1 && i > inner_i.0 && i < inner_i.1) {
                    labeled_pos.push(((i, j), format!("{}{}", grid[i][j - 2], grid[i][j - 1])));
                } else if j == width - 3 || (j == inner_j.0 && i > inner_i.0 && i < inner_i.1) {
                    labeled_pos.push(((i, j), format!("{}{}", grid[i][j + 1], grid[i][j + 2])));
                }
            }
        }
    }

    labeled_pos.sort_by_key(|v| v.1.clone());

    let start = labeled_pos.remove(0).0;
    let end = labeled_pos.pop().unwrap().0;

    for (pos, label) in &labeled_pos {
        for (n_pos, n_label) in &labeled_pos {
            if n_pos == pos {
                continue;
            }
            if label == n_label {
                if pos.0 == 2 || pos.0 == height - 3 || pos.1 == 2 || pos.1 == width - 3 {
                    outer_jumps.insert(*pos, *n_pos);
                } else {
                    inner_jumps.insert(*pos, *n_pos);
                }
            }
        }
    }
    (outer_jumps, inner_jumps, start, end)
}

fn get_inner_bounds(
    grid: &Vec<Vec<char>>,
    width: &usize,
    height: &usize,
) -> ((usize, usize), (usize, usize)) {
    let mut inner_j = (0, 0);
    let mut inner_i = (0, 0);
    let mut a = false;
    let mut b = false;
    let mut k = 1;
    while !a || !b {
        if !a && (grid[height / 2][width / 2 + k] == '.' || grid[height / 2][width / 2 + k] == '#')
        {
            inner_j = (width / 2 - k, width / 2 + k);
            a = true;
        }
        if !b && (grid[height / 2 + k][width / 2] == '.' || grid[height / 2 + k][width / 2] == '#')
        {
            inner_i = (height / 2 - k, height / 2 + k);
            b = true;
        }
        k += 1
    }
    (inner_i, inner_j)
}
