use std::time::Instant;

mod game;
mod part1;
mod part2;

fn main() {
    let instant = Instant::now();
    let input = include_str!("./input/part1.txt");
    let part1_sum = part1::solution(input);
    println!("[1] Sum: {} ({:?})", part1_sum, instant.elapsed());

    let instant = Instant::now();
    let input = include_str!("./input/part1.txt");
    let part2_sum = part2::solution(input);
    println!("[2] Sum: {} ({:?})", part2_sum, instant.elapsed());
}
