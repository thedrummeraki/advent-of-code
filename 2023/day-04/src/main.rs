use std::time::Instant;

mod part1;
mod part2;

fn main() {
    let instant = Instant::now();
    let input = include_str!("./input.txt");
    let solution = part1::solution::execute(input);

    println!("[1] Result: {} ({:?})", solution, instant.elapsed());

    let instant = Instant::now();
    let input = include_str!("./input.txt");
    let solution = part2::solution::execute(input);

    println!("[2] Result: {} ({:?})", solution, instant.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./test.txt");
        let solution = part1::solution::execute(input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("./test.txt");
        let solution = part2::solution::execute(input);
        assert_eq!(solution, 30);
    }
}
