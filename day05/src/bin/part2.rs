use std::ops::Range;
#[derive(Debug)]
struct Map {
    modifier: i64,
    destination: Range<i64>,
}
impl Map {
    fn from(vector: Vec<i64>) -> Self {
        Map {
            modifier: (vector[0] - vector[1]),
            destination: (vector[1]..vector[1] + vector[2]),
        }
    }
    fn convert_number(&self, number: i64) -> i64 {
        if self.destination.contains(&number) {
            return number + self.modifier;
        }
        number
    }
}
#[derive(Debug)]
struct Parser {
    seeds: Vec<i64>,
    maps: Vec<Vec<Map>>,
    actual_seeds: Vec<Range<i64>>,
}
impl Parser {
    fn new() -> Self {
        Parser {
            seeds: (Vec::new()),
            maps: (Vec::new()),
            actual_seeds: (Vec::new()),
        }
    }
    fn actual_seeds(&mut self) {
        for i in (0..self.seeds.len()).step_by(2) {
            let start = self.seeds[i];
            let range = self.seeds[i + 1];
            self.actual_seeds.push(start..start + range);
        }
    }
    fn set_seeds(&mut self, seeds: Vec<i64>) {
        self.seeds = seeds;
    }
    fn convert_number(&self, input: i64) -> i64 {
        let mut input = input;
        let mut last_modified = input;
        'outer: for map_vector in self.maps.iter() {
            for map in map_vector {
                input = map.convert_number(input);
                if input != last_modified {
                    last_modified = input;
                    continue 'outer;
                }
            }
        }
        input
    }
}

fn main() {
    let mut parser = Parser::new();
    let mut lines = include_str!("input.txt").lines();
    parser.set_seeds(
        lines
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|x| x.parse().ok())
            .collect(),
    );
    parser.actual_seeds();
    println!("{:?}", parser.seeds);
    lines.next();
    lines.next();
    let mut index = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            index += 1;
            continue;
        }
        let result: Vec<i64> = line.split(' ').filter_map(|x| x.parse().ok()).collect();
        if parser.maps.get(index).is_none() {
            parser.maps.push(Vec::new());
        }
        parser.maps[index].push(Map::from(result));
    }
    let mut min = i64::MAX;
    for range in parser.actual_seeds.iter() {
        for seed in range.clone() {
            let seed_val = parser.convert_number(seed);
            min = min.min(seed_val);
        }
    }
    println!("{}", min);
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn first_map() {
        let map = Map::from(vec![50, 98, 2]);
        let result = map.convert_number(99);
        assert_eq!(result, 51)
    }
}
