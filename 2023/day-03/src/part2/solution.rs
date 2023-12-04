#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Number(u32, usize, usize, usize),
    Symbol(char, usize, usize),
}

impl std::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Number(value, x1, x2, y) => {
                value.hash(state);
                x1.hash(state);
                x2.hash(state);
                y.hash(state);
            }
            Self::Symbol(char, x, y) => {
                char.hash(state);
                x.hash(state);
                y.hash(state);
            }
        }
    }
}

impl Element {
    fn is_symbol(&self) -> bool {
        matches!(self, Element::Symbol(_, _, _))
    }

    fn is_number(&self) -> bool {
        matches!(self, Element::Number(_, _, _, _))
    }

    fn value(&self) -> u32 {
        match self {
            Self::Number(value, _, _, _) => *value,
            _ => panic!("only Numbers have values"),
        }
    }

    fn char(&self) -> char {
        match self {
            Self::Symbol(char, _, _) => *char,
            _ => panic!("only Symbols have chars"),
        }
    }

    fn x(&self) -> usize {
        match self {
            Self::Symbol(_, x, _) => *x,
            _ => panic!("only Symbols have x"),
        }
    }

    fn y(&self) -> usize {
        match self {
            Self::Symbol(_, _, y) => *y,
            Self::Number(_, _, _, y) => *y,
        }
    }

    fn x_range(&self) -> Vec<usize> {
        let range = match self {
            Self::Number(_, x_start, x_end, _) => {
                let start = if *x_start > 0 { *x_start - 1 } else { 0 };
                let end = x_end + 2;

                start..end
            }
            Self::Symbol(_, x, _) => min(0, *x)..*x,
        };
        range.collect()
    }

    fn is_below(&self, other: &Element) -> bool {
        let y = self.y();
        let other_y = other.y();

        return y > 0 && other_y == y - 1;
    }

    fn is_above(&self, other: &Element) -> bool {
        let y = self.y();
        let other_y = other.y();

        return other_y == y + 1;
    }

    fn is_one_same_line_as(&self, other: &Element) -> bool {
        return self.y() == other.y();
    }
}

pub fn execute(input: &str) -> u32 {
    let mut elements = Vec::<Element>::new();
    let mut y = 0;
    for line in input.lines() {
        let mut number_buffer = Vec::<char>::new();
        let mut first_number_index: Option<usize> = None;

        for (x, c) in line.char_indices() {
            if c.is_numeric() {
                number_buffer.push(c);
                if first_number_index.is_none() {
                    first_number_index = Some(x);
                }
            } else {
                let number_str: String = number_buffer.iter().collect();
                if !number_str.is_empty() {
                    let number: u32 = number_str.parse().unwrap_or_default();
                    elements.push(Element::Number(
                        number,
                        first_number_index.unwrap(),
                        x - 1,
                        y,
                    ));
                    number_buffer.clear();
                    first_number_index = None;
                }

                if c != '.' {
                    elements.push(Element::Symbol(c, x, y));
                }
            }
        }
        if !number_buffer.is_empty() {
            let number_str: String = number_buffer.iter().collect();
            let number: u32 = number_str.parse().unwrap_or_default();
            elements.push(Element::Number(
                number,
                first_number_index.unwrap(),
                line.chars().count() - 1,
                y,
            ));
        }
        y += 1;
    }

    let numbers = elements.iter().filter(|e| e.is_number());
    let symbols = elements
        .iter()
        .filter(|e| e.is_symbol())
        .collect::<Vec<_>>();

    let mut gear_ratios = Vec::<u32>::new();

    for symbol in symbols.iter() {
        let mut adjacent_numbers = Vec::<Element>::new();
        for number in numbers.clone() {
            if symbol.char() == '*'
                && (symbol.is_above(number)
                    || symbol.is_below(number)
                    || symbol.is_one_same_line_as(number))
            {
                if number.x_range().contains(&symbol.x()) {
                    adjacent_numbers.push(number.clone());
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            let gear_ratio = adjacent_numbers[0].value() * adjacent_numbers[1].value();
            gear_ratios.push(gear_ratio);
        }
    }

    gear_ratios.iter().sum()
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
