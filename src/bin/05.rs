use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace1, newline, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Seeds(Vec<u64>);

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Seeds> {
        preceded(
            tuple((tag("seeds:"), space1)),
            separated_list1(space1, complete::u64),
        )
        .map(Seeds)
        .parse(input)
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    source_start: u64,
    dest_start: u64,
    len: u64,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Map> {
        tuple((
            terminated(complete::u64, space1),
            terminated(complete::u64, space1),
            complete::u64,
        ))
        .map(|(dest, source, len)| Map {
            source_start: source,
            dest_start: dest,
            len,
        })
        .parse(input)
    }

    fn process(&self, val: u64) -> Option<u64> {
        (self.source_start..self.source_start + self.len)
            .contains(&val)
            .then(|| (val - self.source_start) + self.dest_start)
    }
}

#[derive(Debug, PartialEq)]
struct MapSection {
    source: String,
    dest: String,
    map: Vec<Map>,
}

impl MapSection {
    fn parse_header(input: &str) -> IResult<&str, (String, String)> {
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))
            .map(|(s, d): (&str, &str)| (s.to_owned(), d.to_owned()))
            .parse(input)
    }

    fn parse(input: &str) -> IResult<&str, MapSection> {
        let (input, (source, dest)) = terminated(MapSection::parse_header, newline)(input)?;
        let (input, map) = separated_list1(newline, Map::parse)(input)?;

        Ok((input, MapSection { source, dest, map }))
    }

    fn process(&self, val: u64) -> u64 {
        self.map.iter().find_map(|m| m.process(val)).unwrap_or(val)
    }
}

fn parse_inner(input: &str) -> IResult<&str, (Seeds, Vec<MapSection>)> {
    all_consuming(terminated(
        tuple((
            terminated(Seeds::parse, multispace1),
            separated_list1(multispace1, MapSection::parse),
        )),
        newline,
    ))(input)
}

fn parse(input: &str) -> (Seeds, Vec<MapSection>) {
    let (_, res) = parse_inner(input).unwrap();
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    seeds
        .0
        .into_iter()
        .map(|mut seed| {
            for map in maps.iter() {
                seed = map.process(seed);
            }
            seed
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    let seeds = seeds
        .0
        .into_iter()
        .tuples()
        .flat_map(|(start, delta)| start..start + delta)
        .collect::<Vec<_>>();
    seeds
        .into_par_iter()
        .map(|mut seed| {
            for map in maps.iter() {
                seed = map.process(seed);
            }
            seed
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13  141";
        let (input, seeds) = Seeds::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(seeds, Seeds(vec![79, 14, 55, 13, 141]));
    }

    #[test]
    fn test_parse_map() {
        let input = "60 56 37";
        let (input, map) = Map::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            map,
            Map {
                source_start: 56,
                dest_start: 60,
                len: 37
            }
        );
    }

    #[test]
    fn test_parse_map_section() {
        let input = r#"light-to-temperature map:
45 77 23
81 45 19
68 64 13"#;
        let (input, map_section) = MapSection::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            map_section,
            MapSection {
                source: "light".to_string(),
                dest: "temperature".to_string(),
                map: vec![
                    Map {
                        source_start: 77,
                        dest_start: 45,
                        len: 23
                    },
                    Map {
                        source_start: 45,
                        dest_start: 81,
                        len: 19
                    },
                    Map {
                        source_start: 64,
                        dest_start: 68,
                        len: 13
                    }
                ]
            }
        )
    }

    #[test]
    fn test_parse_example() {
        parse(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_map() {
        let map = Map {
            source_start: 53,
            dest_start: 49,
            len: 8,
        };
        assert_eq!(map.process(53), Some(49));
        assert_eq!(map.process(55), Some(51));
        assert_eq!(map.process(49), None);
    }

    #[test]
    fn test_map_section() {
        let input = r#"light-to-temperature map:
45 77 23
81 45 19
68 64 13"#;
        let (_, map_section) = MapSection::parse(input).unwrap();
        assert_eq!(map_section.process(77), 45);
        assert_eq!(map_section.process(50), 86);
        assert_eq!(map_section.process(66), 70);
        assert_eq!(map_section.process(2000), 2000);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
