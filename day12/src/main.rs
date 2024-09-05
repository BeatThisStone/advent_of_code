#[derive(Debug, PartialEq, Eq)]
enum Status {
    Damaged,
    Unknown,
}
impl Status {
    fn to_num(&self) -> i32 {
        if &Status::Damaged == self {
            return 1;
        }
        0
    }
}
#[derive(Debug)]
struct HotSpring {
    row: Vec<Status>,
    group: Vec<i32>,
}
impl HotSpring {
    fn new(row: &str, group: Vec<i32>) -> Self {
        let mut vec: Vec<Status> = Vec::new();
        for char in row.chars() {
            vec.push(match char {
                '#' => Status::Damaged,
                '?' => Status::Unknown,
                _ => continue,
            });
        }
        HotSpring { row: (vec), group }
    }
    fn calculate_arrangements(&self) {
        let num = format!("{:b}", self.group[0]);
        println!("{num} lolz");
    }
    fn get_unknown(&self) -> i32 {
        let mut counter = 0;
        for status in self.row.iter() {
            if *status == Status::Unknown {
                counter += 1;
            }
        }
        counter
    }
}
fn main() {
    let mut counter = 0;
    for line in include_str!("input.txt").lines() {
        let breakpoint = line.find(' ').unwrap();
        let (row, group) = line.split_at(breakpoint);
        let group: Vec<i32> = group
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let hot_spring = HotSpring::new(row, group);
        hot_spring.calculate_arrangements();
        println!("{:?}", hot_spring);
    }
}
fn convert_status_vector(vector: Vec<Status>) -> i32 {
    let mut out = 0;
    for i in 0..vector.len() {
        out *= 10;
        out += vector[i].to_num();
    }
    out
}
