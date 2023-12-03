use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    pub fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            c => panic!("Invalid selection: {c}"),
        }
    }

    pub fn our_choice(&self, c: char) -> Self {
        match c {
            'X' => self.lose_to(),
            'Y' => self.clone(),
            'Z' => self.win_to(),
            c => panic!("Invalid choice: {c}"),
        }
    }

    pub fn wins_against(&self, other: &Self) -> bool {
        match self {
            Self::Rock => other.is_scissors(),
            Self::Paper => other.is_rock(),
            Self::Scissors => other.is_paper(),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn is_rock(&self) -> bool {
        match self {
            Self::Rock => true,
            _ => false,
        }
    }

    fn is_paper(&self) -> bool {
        match self {
            Self::Paper => true,
            _ => false,
        }
    }

    fn is_scissors(&self) -> bool {
        match self {
            Self::Scissors => true,
            _ => false,
        }
    }

    fn lose_to(&self) -> Self {
        if self.is_paper() {
            Self::Rock
        } else if self.is_rock() {
            Self::Scissors
        } else {
            Self::Paper
        }
    }

    fn win_to(&self) -> Self {
        if self.is_paper() {
            Self::Scissors
        } else if self.is_rock() {
            Self::Paper
        } else {
            Self::Rock
        }
    }
}

struct Game {
    ours: Selection,
    theirs: Selection,
}

impl Game {
    pub fn from_str(str: &str) -> Self {
        let chars = str.chars().collect::<Vec<_>>();
        let first_char = chars[0];
        let last_char = chars[2];

        let theirs = Selection::from_char(first_char);
        let ours = theirs.our_choice(last_char);

        Self { ours, theirs }
    }

    pub fn score(&self) -> u32 {
        if self.ours.wins_against(&self.theirs) {
            self.ours.score() + 6
        } else if self.theirs.wins_against(&self.ours) {
            self.ours.score()
        } else {
            self.ours.score() + 3
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "US ({:?}) - THEM ({:?}) ==> {}",
            self.ours,
            self.theirs,
            self.score()
        ))
    }
}

pub fn execute(input: &str) -> u32 {
    input
        .lines()
        .map(|str| {
            let game = Game::from_str(str);
            let score = game.score();
            // println!("Game: {game:?}");
            score
        })
        .sum()
}
