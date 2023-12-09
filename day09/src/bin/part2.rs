fn calculate_next(vector: &Vec<i32>) -> i32 {
    let mut differences: Vec<i32> = Vec::new();
    for i in 0..vector.len() {
        if let Some(num) = vector.get(i + 1) {
            differences.push(num - vector[i])
        }
    }
    let mut last = *vector.last().unwrap();
    let mut done = true;
    for num in differences.iter() {
        if *num != 0 {
            done = false;
            break;
        }
    }
    if !done {
        last += calculate_next(&differences);
    }
    last
}

fn main() {
    let lines: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse().unwrap())
                .rev()
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut sum = 0;
    for line in lines {
        sum += calculate_next(&line);
    }
    println!("{}", sum)
}
