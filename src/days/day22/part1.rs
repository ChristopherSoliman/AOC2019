#[derive(Debug)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Increment(i32),
}

const DECK_SIZE: usize = 10007;

pub fn part1(path: &str) -> u32 {
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
    let mut deck: [u32; DECK_SIZE] = core::array::from_fn(|i| i as u32);
    for op in shuffles {
        deck = shuffle(&deck, &op);
    }
    for i in 0..DECK_SIZE {
        if deck[i] == 2019 {
            return i as u32;
        }
    }
    panic!("No card found");
}

fn shuffle(deck: &[u32; DECK_SIZE], shuffle_type: &Shuffle) -> [u32; DECK_SIZE] {
    let mut new_deck = *deck;
    match shuffle_type {
        Shuffle::NewStack => {
            new_deck.reverse();
        }
        Shuffle::Cut(n) => {
            for i in 0..DECK_SIZE {
                new_deck[((DECK_SIZE as i32 - n) as usize + i) % DECK_SIZE] = deck[i];
            }
        }
        Shuffle::Increment(n) => {
            for i in 0..DECK_SIZE {
                new_deck[(i * *n as usize) % DECK_SIZE] = deck[i];
            }
        }
    }
    new_deck
}
