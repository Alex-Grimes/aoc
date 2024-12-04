advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    let out = parsed
        .into_iter()
        .filter(|levels| safe_levels(levels))
        .count();
    Some(out)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    let out = parsed
        .into_iter()
        .filter(|levels| problem_dampener(levels))
        .count();
    Some(out)
}

fn parse_line(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .filter_map(|num| num.parse::<u8>().ok())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(parse_line).collect()
}

fn larger_or_smaller(levels: &[u8]) -> bool {
    let increasing = levels[1] > levels[0];
    for pair in levels.windows(2) {
        if pair[1] > pair[0] && !increasing {
            return false;
        }
        if pair[1] < pair[0] && increasing {
            return false;
        }
    }
    true
}

fn level_check(levels: &[u8]) -> bool {
    levels.windows(2).all(|pair| {
        let diff = pair[1].abs_diff(pair[0]);
        (1..4).contains(&diff)
    })
}

fn safe_levels(levels: &[u8]) -> bool {
    larger_or_smaller(levels) && level_check(levels)
}

fn problem_dampener(levels: &[u8]) -> bool {
    let original = safe_levels(levels);

    if original {
        return true;
    }

    for i in 0..levels.len() {
        let mut clone = levels.to_vec();
        clone.remove(i);
        if safe_levels(&clone) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larger_or_smaller() {
        assert_eq!(larger_or_smaller(&vec![1, 2, 3, 4, 5]), true);
        assert_eq!(larger_or_smaller(&vec![1, 3, 2, 4, 5]), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
