use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, Clone)]
enum Type {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_num().cmp(&other.to_num())
    }
}
impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Type {
    fn to_num(&self) -> i32 {
        match self {
            Type::FiveOfKind => 7,
            Type::FourOfKind => 6,
            Type::FullHouse => 5,
            Type::ThreeOfKind => 4,
            Type::TwoPair => 3,
            Type::OnePair => 2,
            Type::HighCard => 1,
        }
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl Card {
    fn to_num(&self) -> i32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Joker => 1,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_num().cmp(&other.to_num())
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(PartialEq, Eq, Clone)]
struct Hand {
    hand: Vec<Card>,
    bid: i32,
    hand_type: Type,
    rank: i32,
}
impl Hand {
    fn new() -> Self {
        Hand {
            hand: (Vec::new()),
            bid: (0),
            hand_type: (Type::HighCard),
            rank: (0),
        }
    }
    fn set_bid(&mut self, num: i32) {
        self.bid = num;
    }
    fn set_rank(&mut self, rank: usize) {
        self.rank = rank as i32;
    }
    fn set_hand(&mut self, str: &str) {
        for char in str.chars() {
            let card = match char {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Joker,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => continue,
            };
            self.hand.push(card);
        }
    }
    fn calculate_type(&mut self) {
        let mut map: HashMap<Card, i32> = HashMap::new();
        let mut num_of_jokers = 0;
        for card in self.hand.iter() {
            if card == &Card::Joker {
                num_of_jokers += 1;
                continue;
            }
            match map.get(card) {
                Some(val) => map.insert(card.clone(), val + 1),
                None => map.insert(card.clone(), 1),
            };
        }
        if num_of_jokers == 5 {
            self.hand_type = Type::FiveOfKind;
            return;
        }
        match map.values().max().unwrap() + num_of_jokers {
            5 => self.hand_type = Type::FiveOfKind,
            4 => self.hand_type = Type::FourOfKind,
            3 => {
                if map.len() == 3 {
                    self.hand_type = Type::ThreeOfKind
                } else {
                    self.hand_type = Type::FullHouse
                }
            }
            2 => {
                if map.len() == 3 {
                    self.hand_type = Type::TwoPair
                } else {
                    self.hand_type = Type::OnePair
                }
            }
            1 => self.hand_type = Type::HighCard,
            _ => (),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => (),
        }
        for i in 0..self.hand.len() {
            match self.hand[i].to_num().cmp(&other.hand[i].to_num()) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let mut hands_vec: Vec<Hand> = Vec::new();
    include_str!("input.txt").lines().for_each(|x| {
        let mut hand = Hand::new();
        let x = x.split(' ').collect::<Vec<&str>>();
        hand.set_hand(x[0]);
        hand.set_bid(x[1].parse().unwrap());
        hand.calculate_type();
        hands_vec.push(hand);
    });
    hands_vec.sort();
    for (i, hand) in hands_vec.iter_mut().enumerate() {
        hand.set_rank(i + 1);
    }
    let mut sum = 0;
    for card in hands_vec.iter() {
        sum += card.rank * card.bid
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Hand;

    #[test]
    fn hand_cmp() {
        let mut hand1 = Hand::new();
        hand1.set_hand("22222");
        hand1.calculate_type();

        let mut hand2 = Hand::new();
        hand2.set_hand("AAAA2");
        hand2.calculate_type();
        assert_eq!(Ordering::Greater, hand1.cmp(&hand2))
    }

    #[test]
    fn hand_array_sort() {
        let mut hand1 = Hand::new();
        hand1.set_hand("22222");
        hand1.calculate_type();
        let mut vec: Vec<Hand> = Vec::new();
        let mut vec2: Vec<Hand> = Vec::new();
        let mut hand2 = Hand::new();
        hand2.set_hand("AAAA2");
        hand2.calculate_type();
        vec.push(hand1.clone());
        vec.push(hand2.clone());
        vec.sort();
        vec2.push(hand2);
        vec2.push(hand1);
        assert!(vec == vec2);
    }
}
