use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn adjacents(self) -> Vec<Coordinate> {
        (-1..=1)
            .into_iter()
            .flat_map(|x| (-1..=1).into_iter().flat_map(move |y| self.shift(x, y)))
            .collect()
    }

    fn shift(self, x: i32, y: i32) -> Option<Coordinate> {
        if (x, y) == (0, 0) {
            return None;
        }
        let new_x = self.x.checked_add_signed(x as isize)?;
        let new_y = self.y.checked_add_signed(y as isize)?;

        Some((new_x, new_y).into())
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn line(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    if start.y != end.y {
        panic!("Not a horizonal line");
    }
    (start.x..=end.x)
        .into_iter()
        .map(move |x| (x, start.y).into())
        .collect()
}

fn adjacent_coords(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    let line = line(start, end);
    line.iter()
        .flat_map(|c| c.adjacents())
        .filter(|c| !line.contains(c))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Element {
    PartNumber {
        value: u32,
        start: Coordinate,
        end: Coordinate,
    },
    Symbol {
        character: char,
        coordinate: Coordinate,
    },
    Empty {
        coordinate: Coordinate,
    },
}

impl Element {
    fn adjacent_coords(&self) -> Vec<Coordinate> {
        match self {
            Element::PartNumber { start, end, .. } => adjacent_coords(*start, *end),
            Element::Symbol { coordinate, .. } => adjacent_coords(*coordinate, *coordinate),
            Element::Empty { coordinate } => adjacent_coords(*coordinate, *coordinate),
        }
    }

    fn is_at(&self, coordinate: Coordinate) -> bool {
        match self {
            Element::PartNumber { start, end, .. } => line(*start, *end).contains(&coordinate),
            Element::Symbol { coordinate: c, .. } => *c == coordinate,
            Element::Empty { coordinate: c } => *c == coordinate,
        }
    }

    fn is_number(&self) -> bool {
        matches!(self, Element::PartNumber { .. })
    }

    fn is_symbol(&self) -> bool {
        matches!(self, Element::Symbol { .. })
    }

    fn is_empty(&self) -> bool {
        matches!(self, Element::Empty { .. })
    }

    fn is_gear(&self) -> bool {
        if let Element::Symbol { character, .. } = self {
            character == &'*'
        } else {
            false
        }
    }

    fn coord(&self) -> Coordinate {
        match self {
            Element::PartNumber { start, .. } => *start,
            Element::Symbol { coordinate, .. } => *coordinate,
            Element::Empty { coordinate } => *coordinate,
        }
    }
}

#[derive(Debug)]
struct Map {
    elements: Vec<Element>,
}

impl Map {
    fn new(input: &str) -> Self {
        let elements = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                let mut row = Vec::new();
                let mut current_digit: Option<(usize, usize)> = None;
                let chars: Vec<_> = line.chars().collect();

                for i in 0..line.len() {
                    let coordinate = Coordinate { x: i, y };
                    let current_char = chars[i];
                    if current_char.is_digit(10) {
                        current_digit = match current_digit {
                            Some((start, _)) => Some((start, i)),
                            None => Some((i, i)),
                        };
                        if i == line.len() - 1 {
                            let Some((x1, x2)) = current_digit else {
                                panic!("Invalid digit");
                            };
                            let digit_slice = &line[x1..=x2];
                            let value = digit_slice
                                .parse::<u32>()
                                .expect(&format!("Not a number: {digit_slice}"));
                            row.push(Element::PartNumber {
                                value,
                                start: (x1, y).into(),
                                end: (x2, y).into(),
                            });
                            current_digit = None;
                        }
                        continue;
                    }

                    if let Some((x1, x2)) = current_digit {
                        let digit_slice = &line[x1..=x2];
                        let value = digit_slice
                            .parse::<u32>()
                            .expect(&format!("Not a number: {digit_slice}"));
                        row.push(Element::PartNumber {
                            value,
                            start: (x1, y).into(),
                            end: (x2, y).into(),
                        });
                        current_digit = None;
                    }

                    let element = {
                        if current_char == '.' {
                            Element::Empty { coordinate }
                        } else {
                            Element::Symbol {
                                character: current_char,
                                coordinate,
                            }
                        }
                    };
                    row.push(element);
                }

                row
            })
            .collect();

        Self { elements }
    }

    fn get(&self, coordinate: Coordinate) -> Option<&Element> {
        self.elements.iter().find(|el| el.is_at(coordinate))
    }

    fn adjacent_elements(&self, coordinate: Coordinate) -> Vec<&Element> {
        let Some(el) = self.get(coordinate) else {
            return vec![];
        };

        el.adjacent_coords()
            .into_iter()
            .flat_map(|c| {
                let el = self.get(c)?;
                if el.is_empty() {
                    None
                } else {
                    return Some(el);
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn numbers(&self) -> Vec<&Element> {
        self.elements.iter().filter(|el| el.is_number()).collect()
    }

    fn gears(&self) -> Vec<&Element> {
        self.elements.iter().filter(|el| el.is_gear()).collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    Some(
        map.numbers()
            .into_iter()
            .flat_map(|el| {
                let Element::PartNumber { value, .. } = el else {
                    panic!("Not a number")
                };
                if map
                    .adjacent_elements(el.coord())
                    .into_iter()
                    .any(|el| el.is_symbol())
                {
                    Some(*value)
                } else {
                    // dbg!(el);
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    Some(
        map.gears()
            .into_iter()
            .flat_map(|el| {
                let nums = map
                    .adjacent_elements(el.coord())
                    .into_iter()
                    .flat_map(|el| {
                        if let Element::PartNumber { value, .. } = el {
                            Some(*value)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                if nums.len() == 2 {
                    Some(nums.into_iter().product::<u32>())
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
