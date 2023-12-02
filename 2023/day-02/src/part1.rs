use crate::game::Game;

pub fn solution(input: &str) -> u32 {
    let games = input
        .lines()
        .map(|game_str| Game::from_str(game_str))
        .collect::<Vec<_>>();

    let matching_games = games
        .iter()
        .filter_map(|game| {
            if game
                .cube_sets
                .iter()
                .all(|cube_set| cube_set.is_valid_part1())
            {
                Some(game.id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    matching_games
        .into_iter()
        .reduce(|acc, game| acc + game)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = include_str!("./test/part1.txt");
        assert_eq!(solution(test_input), 8);
    }
}
