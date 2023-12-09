fn main() {
    let my_str = include_str!("input.txt");
    let mut found_number: Vec<u32> = Vec::new();
    let mut sum = 0;
    for line in my_str.lines() {
        found_number.clear();
        for char in line.chars() {
            if let Some(num) = char.to_digit(10) {
                found_number.push(num);
            }
        }
        let num: i32 = format!(
            "{}{}",
            found_number.first().unwrap(),
            found_number.last().unwrap()
        )
        .parse()
        .unwrap();
        sum += num;
    }
    println!("{}", sum);
}
