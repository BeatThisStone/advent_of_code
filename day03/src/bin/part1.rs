use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate(x, y)
    }
    fn adjacent_coordinates(&self) -> Vec<Coordinate> {
        let x = self.0 as i32;
        let y = self.1 as i32;
        let mut vector: Vec<Coordinate> = Vec::new();
        for x_modifier in -1..=1 {
            for y_modifier in -1..=1 {
                if x_modifier == 0 && y_modifier == 0 {
                    continue;
                }
                let x: usize = match (x + x_modifier).try_into() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let y: usize = match (y + y_modifier).try_into() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                vector.push(Coordinate::new(x, y));
            }
        }
        vector
    }
}
#[derive(Debug)]
struct MatrixNumber {
    val: i32,
    coordinates: Vec<Coordinate>,
}
impl MatrixNumber {
    fn new(val: i32, coordinate: Coordinate) -> Self {
        MatrixNumber {
            val,
            coordinates: (vec![coordinate]),
        }
    }
    fn contains(&self, coordinate: Coordinate) -> bool {
        self.coordinates.contains(&coordinate)
    }
    fn add_to_self(&mut self, other: Self) {
        self.val = self.val * 10 + other.val;
        self.coordinates.push(other.coordinates[0]);
    }
    fn get_coordinates(&self) -> HashSet<Coordinate> {
        self.coordinates.clone().into_iter().collect()
    }
    fn get_value(&self) -> i32 {
        self.val
    }
}

fn main() {
    let my_str = include_str!("input.txt").lines();
    let mut m_numbers: Vec<MatrixNumber> = Vec::new();
    let mut search_for: HashSet<Coordinate> = HashSet::new();
    for (y, line) in my_str.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if let Some(val) = char.to_digit(10) {
                let val = val as i32;
                let matrix_number = MatrixNumber::new(val, Coordinate::new(x, y));
                if x == 0 {
                    m_numbers.push(matrix_number);
                    continue;
                }
                if let Some(matrix_number_in_vec) =
                    number_in_vector_contains_coordinate(&mut m_numbers, Coordinate::new(x - 1, y))
                {
                    matrix_number_in_vec.add_to_self(matrix_number);
                } else {
                    m_numbers.push(matrix_number)
                }
            } else if char == '.' {
            } else {
                for coordinate in Coordinate::new(x, y).adjacent_coordinates() {
                    search_for.insert(coordinate);
                }
            }
        }
    }
    let mut sum = 0;
    for matrix_number in m_numbers.iter() {
        if !matrix_number
            .get_coordinates()
            .intersection(&search_for)
            .collect::<Vec<&Coordinate>>()
            .is_empty()
        {
            sum += matrix_number.get_value();
        }
    }
    println!("{}", sum)
}

fn number_in_vector_contains_coordinate(
    vector: &mut [MatrixNumber],
    coordinate: Coordinate,
) -> Option<&mut MatrixNumber> {
    vector
        .iter_mut()
        .find(|matrix_number| matrix_number.contains(coordinate))
}
