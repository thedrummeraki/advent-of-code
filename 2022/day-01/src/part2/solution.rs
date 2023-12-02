use std::fmt::Debug;

#[derive(Default, PartialEq, Eq)]
pub struct Elf {
    pub calories: Vec<u32>,
}

impl Debug for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Elt[{} <-- {:?}]",
            self.total_calories(),
            self.calories
        ))
    }
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

    elves.push(current_elf);

    elves.sort_by(|a, b| b.cmp(a));
    let max_elves = elves.iter().take(3).collect::<Vec<_>>();

    max_elves.iter().map(|elf| elf.total_calories()).sum()
}
