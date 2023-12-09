use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let instructions = lines.next().unwrap().trim();
    lines.next();
    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();
        let key = line[0];
        let left = line[2]
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(',')
            .unwrap();
        let right = line[3].strip_suffix(')').unwrap();
        map.insert(key, (left, right));
    }
    let mut instructions = instructions.chars().cycle();
    let mut counter = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        counter += 1;
        if instructions.next().unwrap() == 'L' {
            current = map.get(current).unwrap().0;
        } else {
            current = map.get(current).unwrap().1;
        }
    }
    println!("{:?}", map);
    println!("{}", counter)
}
