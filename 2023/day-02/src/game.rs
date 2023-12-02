use regex::Regex;

#[derive(Debug)]
pub enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Cube {
    pub fn from_str(count: u32, name: &str) -> Self {
        match name {
            "red" => Self::Red(count),
            "blue" => Self::Blue(count),
            "green" => Self::Green(count),
            _ => panic!("Invalid cube name: `{}`", name),
        }
    }

    pub fn count(&self) -> u32 {
        match self {
            Self::Red(count) => *count,
            Self::Blue(count) => *count,
            Self::Green(count) => *count,
        }
    }
}

#[derive(Debug)]
pub struct CubeSet {
    pub cubes: Vec<Cube>,
}

impl CubeSet {
    pub fn from_str(str: &str) -> Vec<Self> {
        str.split(';')
            .map(|part| {
                let cube_str_parts = part.split(',');
                let cubes = cube_str_parts
                    .map(|cube_str_part| {
                        let cube_parts = cube_str_part.split_whitespace().collect::<Vec<_>>();
                        let count = cube_parts[0].parse::<u32>().unwrap_or_default();
                        let name = cube_parts[1];

                        Cube::from_str(count, name)
                    })
                    .collect::<Vec<_>>();

                Self { cubes }
            })
            .collect::<Vec<_>>()
    }

    pub fn is_valid_part1(&self) -> bool {
        let mut red_count = 0;
        let mut blue_count = 0;
        let mut green_count = 0;

        for cube in &self.cubes {
            match cube {
                Cube::Red(count) => red_count += count,
                Cube::Blue(count) => blue_count += count,
                Cube::Green(count) => green_count += count,
            }
        }

        red_count <= 12 && blue_count <= 14 && green_count <= 13
    }

    pub fn red_count(&self) -> u32 {
        self.cubes.iter().filter_map(is_red).sum()
    }

    pub fn blue_count(&self) -> u32 {
        self.cubes.iter().filter_map(is_blue).sum()
    }

    pub fn green_count(&self) -> u32 {
        self.cubes.iter().filter_map(is_green).sum()
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn from_str(str: &str) -> Self {
        let re = Regex::new(r":\s").unwrap();
        let parts = re.split(str).collect::<Vec<_>>();

        let id_haystack = parts[0];
        let re = Regex::new(r"\d+").unwrap();
        let id = re
            .captures(id_haystack)
            .and_then(|captures| captures.get(0))
            .and_then(|matches| Some(matches.as_str()))
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or_default();

        let cube_sets = CubeSet::from_str(parts[1]);

        Self { id, cube_sets }
    }

    pub fn power(&self) -> u32 {
        let mut red_count = 0;
        let mut blue_count = 0;
        let mut green_count = 0;

        self.cube_sets.iter().for_each(|cube_set| {
            red_count = max(cube_set.red_count(), red_count);
            blue_count = max(cube_set.blue_count(), blue_count);
            green_count = max(cube_set.green_count(), green_count);
        });
        red_count * blue_count * green_count
    }
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

fn is_red(cube: &Cube) -> Option<u32> {
    match cube {
        Cube::Red(_) => Some(cube.count()),
        _ => None,
    }
}

fn is_blue(cube: &Cube) -> Option<u32> {
    match cube {
        Cube::Blue(_) => Some(cube.count()),
        _ => None,
    }
}

fn is_green(cube: &Cube) -> Option<u32> {
    match cube {
        Cube::Green(_) => Some(cube.count()),
        _ => None,
    }
}
