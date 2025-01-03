use std::collections::HashMap;
pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut cache: HashMap<&str, u32> = HashMap::new();

    for line in input.lines() {
        let (center, orbit) = line.split_once(")").unwrap();
        orbits
            .entry(&center)
            .and_modify(|v| v.push(orbit))
            .or_insert(vec![orbit]);
        orbits.entry(&orbit).or_insert(vec![]);
    }

    orbits
        .iter()
        .map(|obj| get_objects(&orbits, &obj.0, &mut cache))
        .sum()
}

fn get_objects<'a>(
    orbits: &HashMap<&'a str, Vec<&str>>,
    object: &'a str,
    cache: &mut HashMap<&'a str, u32>,
) -> u32 {
    if let Some(v) = cache.get(&object) {
        return *v;
    }
    let mut sum = 0;

    if object == "COM" {
        return sum;
    }

    for orbit in orbits {
        if orbit.1.contains(&object) {
            sum += 1 + get_objects(orbits, orbit.0, cache);
            break;
        }
    }
    cache.insert(object, sum);
    sum
}
