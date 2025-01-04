use std::collections::HashMap;

#[derive(Debug)]
struct OrePair<'a> {
    name: &'a str,
    quantity: u32,
}

#[derive(Debug)]
struct Reaction<'a> {
    ins: Vec<OrePair<'a>>,
    out: OrePair<'a>,
}

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let dep: HashMap<&str, Reaction> = input
        .lines()
        .map(|line| {
            let (ins, outs) = line.split_once(" => ").unwrap();
            let outs = outs.trim().split_whitespace().collect::<Vec<_>>();
            let out = OrePair {
                name: outs[1],
                quantity: outs[0].trim().parse::<u32>().unwrap(),
            };
            let in_ores = ins
                .split(", ")
                .map(|ore_pair| {
                    let ore_pair = ore_pair.trim().split_whitespace().collect::<Vec<_>>();
                    OrePair {
                        name: ore_pair[1],
                        quantity: ore_pair[0].trim().parse::<u32>().unwrap(),
                    }
                })
                .collect::<Vec<_>>();

            (outs[1], Reaction { ins: in_ores, out })
        })
        .collect::<HashMap<_, _>>();

    let mut q = vec![("FUEL", 1)];
    let mut rem: HashMap<&str, u32> = HashMap::new();

    let mut sum = 0;
    while !q.is_empty() {
        let (req, mut amount) = q.remove(0);
        if req == "ORE" {
            sum += amount;
            continue;
        }

        let reaction = dep.get(&req).unwrap();
        if let Some(r) = rem.get_mut(&req) {
            if *r <= amount {
                amount -= *r;
                *r = 0;
            } else {
                *r = *r - amount;
                amount = 0;
            }
        }
        let mult = amount / reaction.out.quantity + (amount % reaction.out.quantity != 0) as u32;
        let remainder = reaction.out.quantity * mult - amount;
        rem.entry(&req)
            .and_modify(|v| *v += remainder)
            .or_insert(remainder);

        for reactant in &reaction.ins {
            let r_req = reactant.quantity * mult;
            q.push((reactant.name, r_req));
        }
    }
    sum
}
