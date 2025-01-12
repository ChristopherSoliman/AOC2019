#[derive(Debug)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Increment(u64),
}

const DECK_SIZE: u64 = 119315717514047;
const REPS: u64 = 101741582076661;

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let shuffles = input
        .lines()
        .map(|l| {
            let words = l.split_whitespace().collect::<Vec<_>>();
            if words.len() == 2 {
                return Shuffle::Cut(words[1].parse().unwrap());
            }
            if words[1] == "with" {
                return Shuffle::Increment(words[3].parse().unwrap());
            }
            Shuffle::NewStack
        })
        .collect::<Vec<_>>();

    let x = 2020;
    let y = call_reverse(x, &shuffles);
    let z = call_reverse(y, &shuffles);

    let a = ((z as i128 + DECK_SIZE as i128 - y as i128)
        * mod_inv(y + DECK_SIZE - x, DECK_SIZE) as i128
        % DECK_SIZE as i128) as u64;
    let b = ((y as i128 - (a as i128 * x as i128)) % DECK_SIZE as i128) as i64;

    let a_pow = mod_pow(a, REPS, DECK_SIZE);
    let mut v =
        (a_pow as i128 - 1_i128) * mod_inv(a as u64 - 1, DECK_SIZE) as i128 % DECK_SIZE as i128;
    v = (v as i128 * b as i128) % DECK_SIZE as i128;
    v = (v as i128 + a_pow as i128 * x as i128) % DECK_SIZE as i128;
    (v % DECK_SIZE as i128) as u64
}

fn mod_pow(base: u64, exp: u64, modulo: u64) -> u64 {
    if modulo == 1 {
        return 0;
    }
    let mut exp = exp;
    let mut base = base % modulo;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulo as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulo as u128) as u64;
    }
    result
}

fn call_reverse(i: u64, shuffles: &Vec<Shuffle>) -> u64 {
    let mut y = i;
    for op in shuffles.iter().rev() {
        y = new_idx(y, &op);
    }
    y
}

fn new_idx(idx: u64, shuffle_type: &Shuffle) -> u64 {
    match shuffle_type {
        Shuffle::NewStack => DECK_SIZE - 1 - idx,
        Shuffle::Cut(n) => (idx as i64 + *n as i64).rem_euclid(DECK_SIZE as i64) as u64,
        Shuffle::Increment(n) => {
            return (mod_inv(*n, DECK_SIZE) as u128 * idx as u128 % DECK_SIZE as u128) as u64;
        }
    }
}

fn egcd(a: i64, b: i64) -> (u64, u64, u64) {
    let (mut prev_x, mut x) = (1, 0);
    let (mut prev_y, mut y) = (0, 1);
    let (mut prev_r, mut r) = (a, b);

    while r != 0 {
        let q = prev_r.div_euclid(r);
        let temp = (r, x, y);
        x = prev_x - q * x;
        y = prev_y - q * y;
        r = prev_r - q * r;
        (prev_r, prev_x, prev_y) = temp;
    }
    (
        prev_r.abs() as u64,
        prev_x.rem_euclid(x.abs()) as u64,
        prev_y.rem_euclid(y.abs()) as u64,
    )
}

fn mod_inv(a: u64, b: u64) -> u64 {
    let (gcd, c1, _) = egcd(a as i64, b as i64);
    if gcd != 1 {
        panic!("mod_inv({a}, {b}) not found");
    }
    c1.rem_euclid(b) as u64
}
