use std::net;

#[derive(PartialEq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
    Empty,
}

fn main() {
    let mut map: Vec<Vec<Rock>> = Vec::new();
    for line in include_str!("input.txt").lines() {
        let mut vec = Vec::new();
        for char in line.chars() {
            vec.push(match char {
                'O' => Rock::Round,
                '#' => Rock::Cube,
                _ => Rock::Empty,
            })
        }
        map.push(vec);
    }
    let mut old_map = Vec::new();
    while map != old_map {
        old_map = map.clone();
    }
}
