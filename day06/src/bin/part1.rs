struct Race {
    time: i32,
    record: i32,
}
impl Race {
    fn new(time: i32, record: i32) -> Self {
        Race { time, record }
    }
    fn calculate_wins(&self) -> i32 {
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
                .collect()
        })
        .collect();
    let mut result = 1;
    for i in 0..parser[0].len() {
        let race = Race::new(parser[0][i], parser[1][i]);
        result *= race.calculate_wins();
    }
    println!("{}", result)
}
