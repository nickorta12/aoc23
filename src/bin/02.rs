use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(2);

#[derive(Clone, Copy)]
enum Cube {
    Blue(u32),
    Red(u32),
    Green(u32),
}

impl Cube {
    fn over_max(self) -> bool {
        match self {
            Cube::Blue(x) => x > 14,
            Cube::Red(x) => x > 12,
            Cube::Green(x) => x > 13,
        }
    }
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, digit) = terminated(nom::character::complete::u32, tag(" "))(input)?;
    let (input, cube) = alpha1(input)?;
    let cube = match cube {
        "blue" => Cube::Blue(digit),
        "red" => Cube::Red(digit),
        "green" => Cube::Green(digit),
        _ => panic!("Invalid character"),
    };
    Ok((input, cube))
}

fn cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), cube)(input)
}

fn subsets(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    separated_list1(tag("; "), cubes)(input)
}

fn game(input: &str) -> IResult<&str, (u32, Vec<Vec<Cube>>)> {
    let (input, id) = preceded(tag("Game "), nom::character::complete::u32)(input)?;
    let (input, cube_subsets) = preceded(tag(": "), subsets)(input)?;

    Ok((input, (id, cube_subsets)))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, game) = game(line).unwrap();
                game
            })
            .flat_map(|(id, game)| {
                let invalid_game = game
                    .iter()
                    .any(|subsets| subsets.iter().any(|x| x.over_max()));
                if invalid_game {
                    None
                } else {
                    Some(id)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, (_, cubes)) = game(line).unwrap();
                let (mut blue, mut green, mut red) = (0, 0, 0);
                cubes.iter().for_each(|subset| {
                    for cube in subset {
                        match *cube {
                            Cube::Blue(x) => {
                                if x > blue {
                                    blue = x;
                                }
                            }
                            Cube::Red(x) => {
                                if x > red {
                                    red = x;
                                }
                            }
                            Cube::Green(x) => {
                                if x > green {
                                    green = x;
                                }
                            }
                        }
                    }
                });

                blue * green * red
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
