use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solve(&self) -> u64 {
        let first = (1..self.time)
            .into_iter()
            .find(|t| self.possible_win(t))
            .unwrap();
        let last = (1..self.time)
            .into_iter()
            .rev()
            .find(|t| self.possible_win(t))
            .unwrap();

        last + 1 - first
    }

    fn possible_win(&self, t: &u64) -> bool {
        t * (self.time - t) > self.distance
    }
}

fn num_line<'a>(preceed: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u64>> {
    delimited(
        tuple((tag(preceed), space1)),
        separated_list1(space1, complete::u64),
        newline,
    )
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = num_line("Time:")(input)?;
    let (input, distances) = num_line("Distance:")(input)?;
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();
    Ok((input, races))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .unwrap()
            .1
            .into_iter()
            .fold(1, |result, race| race.solve() * result),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, races) = parse(input).unwrap();
    let (time, distance) =
        races
            .into_iter()
            .fold((String::new(), String::new()), |(time, distance), race| {
                (
                    format!("{time}{}", race.time),
                    format!("{distance}{}", race.distance),
                )
            });
    let race = Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    };
    Some(race.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
