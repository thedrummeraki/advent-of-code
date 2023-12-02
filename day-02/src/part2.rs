use crate::game::Game;

pub fn solution(input: &str) -> u32 {
    let games = input
        .lines()
        .map(|game_str| Game::from_str(game_str))
        .collect::<Vec<_>>();

    games.iter().map(|game| game.power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = include_str!("./test/part1.txt");
        assert_eq!(solution(test_input), 2286);
    }
}
