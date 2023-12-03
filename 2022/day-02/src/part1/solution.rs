enum Selection {
    Rock(bool),
    Paper(bool),
    Scissors(bool),
}

impl Selection {
    pub fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock(c == 'X'),
            'B' | 'Y' => Self::Paper(c == 'Y'),
            'C' | 'Z' => Self::Scissors(c == 'Z'),
            c => panic!("Invalid selection: {c}"),
        }
    }

    pub fn wins_against(&self, other: &Self) -> bool {
        match self {
            Self::Rock(_) => other.is_scissors(),
            Self::Paper(_) => other.is_rock(),
            Self::Scissors(_) => other.is_paper(),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Rock(_) => 1,
            Self::Paper(_) => 2,
            Self::Scissors(_) => 3,
        }
    }

    fn is_rock(&self) -> bool {
        match self {
            Self::Rock(_) => true,
            _ => false,
        }
    }

    fn is_paper(&self) -> bool {
        match self {
            Self::Paper(_) => true,
            _ => false,
        }
    }

    fn is_scissors(&self) -> bool {
        match self {
            Self::Scissors(_) => true,
            _ => false,
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

        let ours = Selection::from_char(last_char);
        let theirs = Selection::from_char(first_char);

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
