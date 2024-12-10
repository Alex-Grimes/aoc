use std::usize;

use strum::IntoEnumIterator;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse(input);
    let sum = matrix
        .items()
        .filter_map(|((x, y), &tile)| (tile == 'X').then_some((x, y)))
        .map(|(x, y)| {
            Direction::iter()
                .filter(|direction| {
                    ['M', 'A', 'S']
                        .into_iter()
                        .try_fold((x, y), |(x, y), letter| {
                            direction
                                .propagate(&matrix, (x, y))
                                .and_then(|(x, y)| (matrix[(x, y)] == letter).then_some((x, y)))
                        })
                        .is_some()
                })
                .count()
        })
        .sum();
    Some(sum)
}

fn parse(input: &str) -> pathfinding::matrix::Matrix<char> {
    let mut matrix = pathfinding::matrix::Matrix::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        ' ',
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            matrix[(x, y)] = c;
        });
    });
    matrix
}

#[derive(strum::EnumIter)]
pub enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    fn propagate(
        &self,
        matrix: &pathfinding::matrix::Matrix<char>,
        (x, y): (usize, usize),
    ) -> Option<(usize, usize)> {
        Some(match self {
            Self::North => (x, y.checked_sub(1)?),
            Self::NorthWest => (x.checked_sub(1)?, y.checked_sub(1)?),
            Self::West => (x.checked_sub(1)?, y),
            Self::SouthWest => (x.checked_sub(1)?, {
                let y = y + 1;
                (y < matrix.rows).then_some(y)?
            }),
            Self::South => (x, {
                let y = y + 1;
                (y < matrix.rows).then_some(y)?
            }),
            Self::SouthEast => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                {
                    let y = y + 1;
                    (y < matrix.rows).then_some(y)?
                },
            ),
            Self::East => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                y,
            ),
            Self::NorthEast => (
                {
                    let x = x + 1;
                    (x < matrix.columns).then_some(x)?
                },
                y.checked_sub(1)?,
            ),
        })
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse(input);
    let count = matrix
        .items()
        .filter_map(|((x, y), &tile)| (tile == 'A').then_some((x, y)))
        .filter(|&(x, y)| {
            [
                (Direction::NorthWest, Direction::SouthEast),
                (Direction::NorthEast, Direction::SouthWest),
            ]
            .into_iter()
            .all(|(dir1, dir2)| {
                dir1.propagate(&matrix, (x, y)).and_then(|(x1, y1)| {
                    dir2.propagate(&matrix, (x, y)).map(|(x2, y2)| {
                        (matrix[(x1, y1)] == 'M' && matrix[(x2, y2)] == 'S')
                            || (matrix[(x1, y1)] == 'S' && matrix[(x2, y2)] == 'M')
                    })
                }) == Some(true)
            })
        })
        .count();

    Some(count)
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
