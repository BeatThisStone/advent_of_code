struct Game {
    red: u32,
    blue: u32,
    green: u32,
    numero: u32,
}
impl Game {
    fn get_num(&self) -> u32 {
        self.numero
    }
    fn new(numero: i32) -> Self {
        Game {
            red: (0),
            blue: (0),
            green: (0),
            numero: (numero as u32),
        }
    }
    fn reset(&mut self) {
        self.red = 0;
        self.blue = 0;
        self.green = 0;
    }
    fn increment_red(&mut self, numero: u32) {
        self.red += numero
    }
    fn increment_blue(&mut self, numero: u32) {
        self.blue += numero
    }
    fn increment_green(&mut self, numero: u32) {
        self.green += numero;
    }
    fn get_red(&self) -> u32 {
        self.red
    }
    fn get_blue(&self) -> u32 {
        self.blue
    }
    fn get_green(&self) -> u32 {
        self.green
    }
}
fn main() {
    let my_str = include_str!("input.txt");
    let mut total: Vec<Game> = Vec::new();
    let mut sum = 0;
    let mut index: i32 = -1;
    'outer: for line in my_str.lines() {
        index += 1;
        let line: Vec<&str> = line.split(":").collect();
        let games: Vec<&str> = line[1].split(";").collect();
        total.push(Game::new(index + 1));
        for game in games {
            total[index as usize].reset();
            let parts: Vec<&str> = game.split(" ").collect();
            for j in 0..parts.len() {
                if let Ok(num) = parts[j].parse() {
                    if parts[j + 1].contains("green") {
                        total[index as usize].increment_green(num);
                    } else if parts[j + 1].contains("red") {
                        total[index as usize].increment_red(num);
                    } else if parts[j + 1].contains("blue") {
                        total[index as usize].increment_blue(num);
                    } else {
                        println!("Errore")
                    }
                }
            }
            let i = index as usize;
            if total[i].get_red() > 12 {
                continue 'outer;
            }
            if total[i].get_blue() > 14 {
                continue 'outer;
            }
            if total[i].get_green() > 13 {
                continue 'outer;
            }
        }
        sum += total[index as usize].get_num();
    }
    println!("{}", sum);
}
