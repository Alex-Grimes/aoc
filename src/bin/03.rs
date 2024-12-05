advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum: u32 = 0;

    for capture in re.captures_iter(input) {
        let num1 = capture[1].parse::<u32>().ok()?;
        let num2 = capture[2].parse::<u32>().ok()?;
        sum = sum.checked_add(num1 * num2)?;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut sum: u32 = 0;

    for capture in re.captures_iter(input) {
        if capture[1].starts_with("mul") {
            if enabled {
                let num1 = capture[2].parse::<u32>().ok()?;
                let num2 = capture[3].parse::<u32>().ok()?;
                sum = sum.checked_add(num1 * num2)?;
            }
        } else if &capture[1] == "do()" {
            enabled = true;
        } else if &capture[1] == "don't()" {
            enabled = false;
        }
    }

    Some(sum)
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
