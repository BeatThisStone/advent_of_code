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
    fn get_points(&self) -> i32 {
        let points = self
            .winning_numbers
            .intersection(&self.numbers)
            .collect::<Vec<&i32>>()
            .len();

        if points > 0 {
            let base: i32 = 2;
            let exp: u32 = (points - 1) as u32;
            base.pow(exp)
        } else {
            0
        }
    }
}

fn main() {
    let mut sum = 0;
    let my_str = include_str!("input.txt");
    for line in my_str.lines() {
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
        sum += card.get_points();
    }
    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Card;

    #[test]
    fn point_of_card() {
        let mut card = Card::new();
        let win: HashSet<i32> = HashSet::from_iter([1, 2, 3].iter().cloned());
        let nums: HashSet<i32> = HashSet::from_iter([2, 3, 4].iter().cloned());
        card.set_winning_numbers(win);
        card.set_numbers(nums);
        assert_eq!(2, card.get_points());
    }
}
