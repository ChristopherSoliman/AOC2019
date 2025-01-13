use std::collections::HashSet;

const HEIGHT: usize = 5;
const WIDTH: usize = 5;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut set: HashSet<u32> = HashSet::new();
    loop {
        let bio = biodiversity(&grid);
        if set.contains(&bio) {
            return bio;
        }
        set.insert(bio);
        grid = next_minute(&grid);
    }
}

fn next_minute(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut n_grid = grid.clone();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut adj_bugs = 0;

            for dir in DIRS {
                let (ni, nj) = (i as i32 + dir.0 as i32, j as i32 + dir.1 as i32);
                if ni < 0 || ni >= HEIGHT as i32 || nj < 0 || nj >= WIDTH as i32 {
                    continue;
                }
                if grid[ni as usize][nj as usize] == '#' {
                    adj_bugs += 1;
                }
            }

            match grid[i][j] {
                '.' if (1..=2).contains(&adj_bugs) => n_grid[i][j] = '#',
                '#' if adj_bugs != 1 => n_grid[i][j] = '.',
                _ => {}
            }
        }
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
