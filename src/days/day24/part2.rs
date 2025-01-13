use std::collections::VecDeque;

const HEIGHT: usize = 5;
const WIDTH: usize = 5;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rec_map: VecDeque<Vec<Vec<char>>> = VecDeque::from(vec![grid.clone()]);
    for _ in 0..200 {
        rec_map = next_minute(&rec_map);
    }
    get_bugs(&rec_map)
}

fn get_bugs(rec_grid: &VecDeque<Vec<Vec<char>>>) -> u32 {
    rec_grid
        .iter()
        .map(|grid| {
            grid.iter()
                .map(|row| row.iter().filter(|c| **c == '#').count() as u32)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn empty_grid() -> Vec<Vec<char>> {
    let mut empty = vec![];
    for _ in 0..HEIGHT {
        let mut t = vec![];
        for _ in 0..WIDTH {
            t.push('.');
        }
        empty.push(t);
    }
    empty
}

fn get_adjacent(rec_grid: &VecDeque<Vec<Vec<char>>>, level: usize, row: usize, col: usize) -> u32 {
    let mut adj_bugs = 0;
    for dir in DIRS {
        let (ni, nj) = (row as i32 + dir.0 as i32, col as i32 + dir.1 as i32);
        if ni < 0 || ni >= HEIGHT as i32 || nj < 0 || nj >= WIDTH as i32 {
            if level == 0 {
                continue;
            }
            if ni < 0 {
                if rec_grid[level - 1][1][2] == '#' {
                    adj_bugs += 1;
                }
            } else if nj < 0 {
                if rec_grid[level - 1][2][1] == '#' {
                    adj_bugs += 1;
                }
            } else if ni >= HEIGHT as i32 {
                if rec_grid[level - 1][3][2] == '#' {
                    adj_bugs += 1;
                }
            } else if nj >= WIDTH as i32 {
                if rec_grid[level - 1][2][3] == '#' {
                    adj_bugs += 1;
                }
            }
        } else if ni == 2 && nj == 2 {
            if level == rec_grid.len() - 1 {
                continue;
            }
            if row == 1 {
                for k in 0..WIDTH {
                    if rec_grid[level + 1][0][k] == '#' {
                        adj_bugs += 1;
                    }
                }
            } else if row == 3 {
                for k in 0..WIDTH {
                    if rec_grid[level + 1][HEIGHT - 1][k] == '#' {
                        adj_bugs += 1;
                    }
                }
            } else if col == 1 {
                for k in 0..HEIGHT {
                    if rec_grid[level + 1][k][0] == '#' {
                        adj_bugs += 1;
                    }
                }
            } else if col == 3 {
                for k in 0..HEIGHT {
                    if rec_grid[level + 1][k][WIDTH - 1] == '#' {
                        adj_bugs += 1;
                    }
                }
            }
        } else if rec_grid[level][ni as usize][nj as usize] == '#' {
            adj_bugs += 1;
        }
    }

    adj_bugs
}

fn next_minute(rec_grid: &VecDeque<Vec<Vec<char>>>) -> VecDeque<Vec<Vec<char>>> {
    let mut rec_grid = rec_grid.clone();
    rec_grid.push_back(empty_grid());
    rec_grid.push_front(empty_grid());

    let mut n_grid = rec_grid.clone();

    for level in 0..rec_grid.len() {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i == 2 && j == 2 {
                    continue;
                }
                let adj_bugs = get_adjacent(&rec_grid, level, i, j);
                match rec_grid[level][i][j] {
                    '.' if (1..=2).contains(&adj_bugs) => n_grid[level][i][j] = '#',
                    '#' if adj_bugs != 1 => n_grid[level][i][j] = '.',
                    _ => {}
                }
            }
        }
    }

    let sum_first = n_grid[0]
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>();
    let sum_last = n_grid[n_grid.len() - 1]
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>();

    if sum_first == 0 {
        n_grid.pop_front();
    }
    if sum_last == 0 {
        n_grid.pop_back();
    }
    n_grid
}

fn biodiversity(grid: &Vec<Vec<char>>) -> u32 {
    let mut b = 0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if grid[i][j] == '#' {
                b |= 1 << (i * WIDTH + j);
            }
        }
    }
    b
}
