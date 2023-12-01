use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                (
                    line.chars().find(|s| s.is_numeric()),
                    line.chars().rev().find(|s| s.is_numeric()),
                )
            })
            .map(|(a, b)| {
                format!("{}{}", a.unwrap(), b.unwrap())
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    Some(
        input
            .lines()
            .map(|line| {
                (
                    map.iter()
                        .filter_map(|(key, value)| line.find(key).map(|index| (index, value)))
                        .fold(
                            (10000_usize, &0_u32),
                            |(acc_index, acc_value), (index, value)| {
                                if index <= acc_index {
                                    (index, value)
                                } else {
                                    (acc_index, acc_value)
                                }
                            },
                        )
                        .1,
                    map.iter()
                        .filter_map(|(key, value)| line.rfind(key).map(|index| (index, value)))
                        .fold((0, &0_u32), |(acc_index, acc_value), (index, value)| {
                            if index >= acc_index {
                                (index, value)
                            } else {
                                (acc_index, acc_value)
                            }
                        })
                        .1,
                )
            })
            .map(|(a, b)| format!("{a}{b}").parse::<u32>().unwrap())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
