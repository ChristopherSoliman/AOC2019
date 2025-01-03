const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part2(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars = input.lines().flat_map(|l| l.chars());

    let mut image: [[u32; WIDTH]; HEIGHT] = [[2; WIDTH]; HEIGHT];

    chars.enumerate().for_each(|(idx, c)| {
        let mod_idx = idx % (WIDTH * HEIGHT);
        let i = mod_idx / WIDTH;
        let j = mod_idx % WIDTH;

        if image[i][j] == 2 {
            image[i][j] = c.to_digit(10).unwrap();
        }
    });

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let c = image[i][j];
            if c == 0 {
                print!(" ");
            } else if c == 1 {
                print!("#");
            }
        }
        println!("");
    }
    0
}
