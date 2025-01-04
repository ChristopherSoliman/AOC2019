use std::collections::HashMap;

#[derive(Debug)]
struct OrePair<'a> {
    name: &'a str,
    quantity: u64,
}

#[derive(Debug)]
struct Reaction<'a> {
    ins: Vec<OrePair<'a>>,
    out: OrePair<'a>,
}

const TARGET: u64 = 1000000000000;

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let dep: HashMap<&str, Reaction> = input
        .lines()
        .map(|line| {
            let (ins, outs) = line.split_once(" => ").unwrap();
            let outs = outs.trim().split_whitespace().collect::<Vec<_>>();
            let out = OrePair {
                name: outs[1],
                quantity: outs[0].trim().parse::<u64>().unwrap(),
            };
            let in_ores = ins
                .split(", ")
                .map(|ore_pair| {
                    let ore_pair = ore_pair.trim().split_whitespace().collect::<Vec<_>>();
                    OrePair {
                        name: ore_pair[1],
                        quantity: ore_pair[0].trim().parse::<u64>().unwrap(),
                    }
                })
                .collect::<Vec<_>>();

            (outs[1], Reaction { ins: in_ores, out })
        })
        .collect::<HashMap<_, _>>();

    let mut lb = 0;
    let mut ub = TARGET;

    while ub != lb {
        let m = lb + (ub - lb) / 2;
        let ore = find_ore(&dep, &m);
        if ore > TARGET {
            ub = m - 1;
        } else if ore < TARGET {
            lb = m + 1;
        } else {
            return m;
        }
    }
    ub
}

fn find_ore(dep: &HashMap<&str, Reaction>, fuel_quantity: &u64) -> u64 {
    let mut q: Vec<(&str, u64)> = vec![("FUEL", *fuel_quantity)];
    let mut rem: HashMap<&str, u64> = HashMap::new();

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
        let mult = amount / reaction.out.quantity as u64
            + (amount % reaction.out.quantity as u64 != 0) as u64;
        let remainder = reaction.out.quantity as u64 * mult - amount;
        rem.entry(&req)
            .and_modify(|v| *v += remainder)
            .or_insert(remainder);

        for reactant in &reaction.ins {
            let r_req = reactant.quantity as u64 * mult as u64;
            q.push((reactant.name, r_req));
        }
    }
    sum
}
