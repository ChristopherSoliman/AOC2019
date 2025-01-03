use std::collections::HashMap;
struct Orbit<'a> {
    level: usize,
    parent: &'a str,
    children: Vec<&'a str>,
}

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (center, orbit) = line.split_once(")").unwrap();
        orbits
            .entry(&center)
            .and_modify(|v| v.push(orbit))
            .or_insert(vec![orbit]);
        orbits.entry(&orbit).or_insert(vec![]);
    }

    let mut level_orbits: HashMap<&str, Orbit> = HashMap::new();
    make_levels(&"COM", &"COM", &orbits, &mut level_orbits, 0);

    let mut me = level_orbits.get(&"YOU").unwrap().parent;
    let mut santa = level_orbits.get(&"SAN").unwrap().parent;
    let mut sum = 0;

    let mut me_depth = level_orbits.get(&me).unwrap().level;
    let mut santa_depth = level_orbits.get(&santa).unwrap().level;

    while me != santa {
        sum += 1;
        if me_depth > santa_depth {
            me = level_orbits.get(&me).unwrap().parent;
            me_depth -= 1;
        } else if me_depth < santa_depth {
            santa = level_orbits.get(&santa).unwrap().parent;
            santa_depth -= 1;
        } else {
            me = level_orbits.get(&me).unwrap().parent;
            santa = level_orbits.get(&santa).unwrap().parent;
            me_depth -= 1;
            santa_depth -= 1;
            sum += 1;
        }
    }

    sum
}

fn make_levels<'a>(
    head: &'a str,
    parent: &'a str,
    orbits: &HashMap<&'a str, Vec<&'a str>>,
    level_orbits: &mut HashMap<&'a str, Orbit<'a>>,
    level: usize,
) {
    let children = orbits.get(head).unwrap();
    level_orbits.insert(
        head,
        Orbit {
            level: level + 1,
            parent,
            children: children.to_vec(),
        },
    );
    for child in children {
        make_levels(&child, &head, orbits, level_orbits, level + 1);
    }
}
