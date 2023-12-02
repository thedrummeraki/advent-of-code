#[derive(Default, PartialEq, Eq)]
pub struct Elf {
    pub calories: Vec<u32>,
}

impl Elf {
    pub fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }

    pub fn append_calories(&mut self, calories: u32) {
        self.calories.push(calories);
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_calories().partial_cmp(&other.total_calories())
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

pub fn execute(input: &str) -> u32 {
    let mut elves: Vec<Elf> = Vec::new();
    let mut current_elf = Elf::default();

    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf::default();
        } else {
            let calories = line.parse::<u32>().unwrap_or_default();
            current_elf.append_calories(calories);
        }
    }

    elves
        .iter()
        .max()
        .map(|elf| elf.total_calories())
        .unwrap_or_default()
}
