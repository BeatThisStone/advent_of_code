use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}
impl Card {
    fn new() -> Self {
        Card {
            winning_numbers: (HashSet::new()),
            numbers: (HashSet::new()),
        }
    }
    fn set_winning_numbers(&mut self, set: HashSet<i32>) {
        self.winning_numbers = set;
    }
    fn set_numbers(&mut self, set: HashSet<i32>) {
        self.numbers = set;
    }

    fn get_wins(&self) -> i32 {
        self.winning_numbers
            .intersection(&self.numbers)
            .collect::<Vec<&i32>>()
            .len() as i32
    }
}

fn main() {
    let mut sum: i32;
    let my_str = include_str!("input.txt");
    let mut card_vector: Vec<i32> = Vec::new();
    let mut index = -1;
    for line in my_str.lines() {
        index += 1;
        let i = index as usize;
        if card_vector.get(i).is_none() {
            card_vector.push(1);
        }
        let card_numbers: Vec<&str> = line.split(':').nth(1).unwrap().split('|').collect();
        let mut card = Card::new();
        card.set_winning_numbers(
            card_numbers[0]
                .trim()
                .split(' ')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect(),
        );
        card.set_numbers(
            card_numbers[1]
                .trim()
                .split(' ')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect(),
        );
        sum = card.get_wins();
        for _ in 0..card_vector[i] {
            strange_vec(&mut card_vector, i, sum);
        }
    }
    let sum: i32 = card_vector.iter().sum();
    println!("{}", sum)
}

fn strange_vec(vector: &mut Vec<i32>, index: usize, sum: i32) {
    let index = (index + 1) as i32;
    for i in index..index + sum {
        let i = i as usize;
        if vector.get(i).is_none() {
            vector.push(1);
        }
        vector[i] += 1;
    }
}
