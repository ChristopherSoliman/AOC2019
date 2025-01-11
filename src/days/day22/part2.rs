use core::panic;

#[derive(Debug)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Increment(i32),
}

//const DECK_SIZE: usize = 119315717514047;
//const REPS: usize = 101741582076661;
const DECK_SIZE: usize = 10007;
const REPS: usize = 1;

pub fn part2(path: &str) -> usize {
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
    let mut deck: Vec<usize> = vec![];
    for i in 0..DECK_SIZE {
        deck.push(i);
    }
    let mut i = 0;
    while i < REPS {
        for op in &shuffles {
            deck = shuffle(&deck, &op);
        }
        i += 1;
    }

    for i in 0..DECK_SIZE {
        if deck[i] == 2019 {
            return i;
        }
    }
    panic!("No solution found");
}

fn shuffle(deck: &Vec<usize>, shuffle_type: &Shuffle) -> Vec<usize> {
    let mut new_deck = deck.clone();
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
