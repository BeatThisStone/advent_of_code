use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let starting_instructions = lines.next().unwrap().trim().chars().cycle();
    lines.next();
    let mut cursors: Vec<&str> = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(' ').collect();
        let key = line[0];
        if key.chars().nth(2).unwrap() == 'A' {
            cursors.push(key);
        }
        let left = line[2]
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(',')
            .unwrap();
        let right = line[3].strip_suffix(')').unwrap();
        map.insert(key, (left, right));
    }
    let mut counter;
    let mut results: Vec<i64> = Vec::new();
    for cursor in cursors.iter_mut() {
        let mut instructions = starting_instructions.clone();
        counter = 0;
        while cursor.chars().nth(2).unwrap() != 'Z' {
            counter += 1;
            if instructions.next().unwrap() == 'L' {
                *cursor = map.get(cursor).unwrap().0;
            } else {
                *cursor = map.get(cursor).unwrap().1;
            }
        }
        results.push(counter);
    }
    let counter: i64 = least_common_multiple(&results);

    println!("{}", counter);
}

fn least_common_multiple(vector: &[i64]) -> i64 {
    let mut vector = vector.to_owned();
    while vector.len() > 1 {
        vector[0] = num::integer::lcm(vector[0], vector.pop().unwrap())
    }
    vector[0]
}
