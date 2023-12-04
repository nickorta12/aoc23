advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
                let first = digits.first().unwrap();
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .sum(),
    )
}

const NAMED_DIGITS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_digits(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    let chars = line.chars().collect::<Vec<_>>();
    for i in 0..line.len() {
        if let Some(digit) = chars[i].to_digit(10) {
            digits.push(digit);
            continue;
        }
        let remain = &line[i..];
        for (name, digit) in NAMED_DIGITS {
            if remain.starts_with(name) {
                digits.push(digit);
                break;
            }
        }
    }
    digits
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digits = get_digits(line);
                let first = digits.first().unwrap();
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
