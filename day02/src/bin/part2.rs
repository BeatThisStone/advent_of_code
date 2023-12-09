struct Game {
    red: u32,
    blue: u32,
    green: u32,
}
impl Game {
    fn new() -> Self {
        Game {
            red: (0),
            blue: (0),
            green: (0),
        }
    }
    fn set_red(&mut self, numero: u32) {
        if self.red < numero || self.red == 0 {
            self.red = numero
        }
    }
    fn set_blue(&mut self, numero: u32) {
        if self.blue < numero || self.blue == 0 {
            self.blue = numero
        }
    }
    fn set_green(&mut self, numero: u32) {
        if self.green < numero || self.green == 0 {
            self.green = numero
        }
    }
    fn get_power(&self) -> u32 {
        let mut rosso = self.red;
        if rosso == 0 {
            rosso = 1;
        }
        let mut verde = self.green;
        if verde == 0 {
            verde = 1;
        }
        let mut blu = self.blue;
        if blu == 0 {
            blu = 1;
        }
        rosso * verde * blu
    }
}
fn main() {
    let my_str = include_str!("input.txt");
    let mut total: Vec<Game> = Vec::new();
    let mut sum = 0;
    let mut index: i32 = -1;
    for line in my_str.lines() {
        index += 1;
        let i = index as usize;
        let line: Vec<&str> = line.split(":").collect();
        let games: Vec<&str> = line[1].split(";").collect();
        total.push(Game::new());
        for game in games {
            let parts: Vec<&str> = game.split(" ").collect();
            for j in 0..parts.len() {
                if let Ok(num) = parts[j].parse() {
                    if parts[j + 1].contains("green") {
                        total[i].set_green(num);
                    } else if parts[j + 1].contains("red") {
                        total[i].set_red(num);
                    } else if parts[j + 1].contains("blue") {
                        total[i].set_blue(num);
                    } else {
                        println!("Errore")
                    }
                }
            }
        }
        sum += total[i].get_power();
    }
    println!("{}", sum);
}
