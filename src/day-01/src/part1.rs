pub fn solution() {
    let str = include_str!("./input.txt");
    let lines = str.lines();
    let mut sum = 0;

    for line in lines {
        let first_digit = line.chars().find_map(char_to_digit).unwrap_or_default();
        let last_digit = line
            .chars()
            .rev()
            .find_map(char_to_digit)
            .unwrap_or_default();

        let value = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap_or_default();
        sum += value;
    }

    println!("[1] Sum: {}", sum);
}

fn char_to_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}
