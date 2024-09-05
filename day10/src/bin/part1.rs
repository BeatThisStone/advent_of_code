use std::cmp::min;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    East,
    South,
    Start,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::Start => Direction::Start,
        }
    }
    fn into_coordinate(&self) -> Option<(i32, i32)> {
        let result = match self {
            Direction::North => (-1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::Start => return None,
        };
        Some(result)
    }
    fn from_coordinate(coordinates: (i32, i32)) -> Direction {
        match coordinates.0 {
            -1 => return Direction::North,
            1 => return Direction::South,
            _ => (),
        }
        match coordinates.1 {
            -1 => Direction::West,
            _ => Direction::East,
        }
    }
}
#[derive(Debug)]
struct Pipe {
    directions: Vec<Direction>,
}
impl Pipe {
    fn from(char: char) -> Self {
        let mut vec = Vec::new();
        match char {
            '|' => {
                vec.push(Direction::North);
                vec.push(Direction::South);
            }
            '-' => {
                vec.push(Direction::West);
                vec.push(Direction::East);
            }
            'L' => {
                vec.push(Direction::North);
                vec.push(Direction::East);
            }
            'J' => {
                vec.push(Direction::North);
                vec.push(Direction::West);
            }
            '7' => {
                vec.push(Direction::West);
                vec.push(Direction::South);
            }
            'F' => {
                vec.push(Direction::East);
                vec.push(Direction::South);
            }
            'S' => vec.push(Direction::Start),
            _ => (),
        };
        Pipe { directions: (vec) }
    }
    fn is_connected_to(&self, pipe: &Pipe) -> bool {
        if self.directions.contains(&Direction::Start) && !pipe.directions.is_empty() {
            return true;
        }
        for direction in self.directions.iter() {
            if pipe.directions.contains(&direction.opposite()) {
                return true;
            }
        }
        false
    }
    fn is_starting_pipe(&self) -> bool {
        self.directions.contains(&Direction::Start)
    }
    fn other_direction(&self, dir: &Direction) -> Direction {
        if self.directions.contains(&Direction::Start) {
            return Direction::Start;
        }
        let mut vec = self.directions.clone();
        vec.remove(vec.iter().position(|r| r == dir).unwrap());
        vec[0]
    }
}
fn main() {
    let mut map: Vec<Vec<Pipe>> = Vec::new();
    let mut starting_position = (0, 0);
    for (y, line) in include_str!("input.txt").lines().enumerate() {
        let mut vec = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                starting_position = (y, x);
            }
            vec.push(Pipe::from(char))
        }
        map.push(vec);
    }
    let mut smallest_path = i32::MAX;
    let starting_y = starting_position.0;
    let mut y = starting_y;
    let starting_x = starting_position.1;
    let mut x = starting_x;
    let mut coming_from = Direction::Start;
    let mut selected_pipe = &map[y][x];
    'outer: for dir in vec![
        Direction::North,
        Direction::West,
        Direction::East,
        Direction::South,
    ] {
        let mut path = 0;
        loop {
            let modifiers = selected_pipe
                .other_direction(&coming_from)
                .into_coordinate()
                .unwrap_or(dir.into_coordinate().unwrap());
            y = match usize::try_from(y as i32 + modifiers.0) {
                Ok(num) => num,
                Err(_) => continue 'outer,
            };
            x = match usize::try_from(x as i32 + modifiers.1) {
                Ok(num) => num,
                Err(_) => continue 'outer,
            };
            if map[y][x].is_starting_pipe() {
                break;
            }
            if !selected_pipe.is_connected_to(&map[y][x]) {
                coming_from = Direction::Start;
                y = starting_y;
                x = starting_x;
                continue 'outer;
            }
            path += 1;
            coming_from = Direction::from_coordinate(modifiers).opposite();
            selected_pipe = &map[y][x];
        }
        if path != 0 {
            smallest_path = min(smallest_path, path);
            break;
        }
    }
    println!("{}", smallest_path);
    println!("{}", smallest_path / 2 + 1);
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Pipe};

    #[test]
    fn what() {
        let pipe1 = Pipe::from('F');
        let pipe2 = Pipe::from('7');
        assert!(pipe1.is_connected_to(&pipe2))
    }

    #[test]
    fn why() {
        let pipe1 = Pipe::from('-');
        let pipe2 = Pipe::from('F');
        assert!(pipe1.is_connected_to(&pipe2));
    }
}
