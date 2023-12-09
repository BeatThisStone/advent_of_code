struct Race {
    time: i64,
    record: i64,
}
impl Race {
    fn new(time: i64, record: i64) -> Self {
        Race { time, record }
    }
    fn calculate_wins(&self) -> i64 {
        let mut wins = 0;
        for i in 0..=self.time {
            let remaining_time = self.time - i;
            let race_result = i * remaining_time;
            if race_result > self.record {
                wins += 1;
            }
        }
        if wins > 0 {
            wins
        } else {
            1
        }
    }
}

fn main() {
    let parser: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut big_time = String::new();
    for num in parser[0].iter() {
        big_time = format!("{}{}", big_time, num);
    }
    let big_time: i64 = big_time.parse().unwrap();

    let mut big_distance = String::new();
    for num in parser[1].iter() {
        big_distance = format!("{}{}", big_distance, num);
    }
    let big_distance: i64 = big_distance.parse().unwrap();
    let race = Race::new(big_time, big_distance);
    println!("{}", race.calculate_wins())
}
