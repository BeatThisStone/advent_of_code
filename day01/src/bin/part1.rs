fn main() {
    let mut found_number: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in include_str!("input.txt").lines() {
        if line.is_empty() {
            break;
        }
        found_number.clear();
        for char in line.chars() {
            if let Some(num) = char.to_digit(10) {
                found_number.push(num as i32);
            }
        }
        let num = found_number.first().unwrap() * 10 + found_number.last().unwrap();
        sum += num;
    }
    println!("{}", sum);
}
