use std::time::Instant;

#[derive(Debug)]
enum Digit {
    String(String, u32),
    Number(u32),
}

impl Digit {
    fn value(&self) -> u32 {
        match self {
            Digit::String(_, value) => *value,
            Digit::Number(value) => *value,
        }
    }
}

pub fn solution() {
    // let str = include_str!("./test-input2.txt");
    let start = Instant::now();
    let str = include_str!("./input.txt");
    let mut sum = 0;

    for line in str.lines() {
        sum += detect_all_digits(line);
    }

    let duration = start.elapsed();
    println!("[2] Sum: {} ({duration:?})", sum);
}

fn detect_all_digits(str: &str) -> u32 {
    let digits_map: [(&str, u32); 10] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ];

    let mut digits: Vec<Digit> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    for c in str.chars() {
        buffer.push(c);

        if c.is_digit(10) {
            digits.push(Digit::Number(c.to_digit(10).unwrap()));
            buffer.clear();
        } else {
            let as_str = buffer.iter().collect::<String>();
            let potential_digits: Vec<_> = digits_map
                .iter()
                .filter_map(|(str, _)| {
                    if str.starts_with(&as_str) {
                        Some(str)
                    } else {
                        None
                    }
                })
                .collect();

            if potential_digits.is_empty() {
                if !buffer.is_empty() {
                    buffer.remove(0);
                }
                continue;
            }

            let digit_info = digits_map.iter().find(|digit| digit.0.eq(&as_str));
            if let Some((str, value)) = digit_info {
                digits.push(Digit::String(str.to_string(), *value));
                buffer.clear();
                buffer.push(c);
            }
        }
    }

    let first_digit = digits
        .first()
        .map(|digit| digit.value())
        .unwrap_or_default();
    let last_digit = digits.last().map(|digit| digit.value()).unwrap_or_default();

    let calibration = format!("{}{}", first_digit, last_digit);
    calibration.parse().unwrap_or_default()
}
