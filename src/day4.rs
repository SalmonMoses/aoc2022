use crate::utils;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge};
use std::convert::identity;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

type Assignment = (RangeInclusive<u64>, RangeInclusive<u64>);

fn parse_file(input: &str) -> nom::IResult<&str, Vec<Assignment>> {
    separated_list1(tag("\r\n"), parse_line)(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, Assignment> {
    let (input, first) = parse_group(&input)?;
    let (input, _) = tag(",")(input)?;
    let (input, second) = parse_group(input)?;
    Ok((input, (first, second)))
}

fn parse_group(input: &str) -> nom::IResult<&str, RangeInclusive<u64>> {
    let (input, start) = map_res(digit1, u64::from_str)(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, finish) = map_res(digit1, u64::from_str)(input)?;
    Ok((input, start..=finish))
}

fn fully_contains(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

pub fn task_a<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    let (_, assignments) = parse_file(&input).unwrap();
    Ok(assignments
        .par_iter()
        .map(|(first, second)| fully_contains(first, second) || fully_contains(second, first))
        .filter(|b| *b)
        .count() as u64)
}

pub fn task_b<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    let (_, assignments) = parse_file(&input).unwrap();
    Ok(assignments
        .par_iter()
        .map(|(first, second)| overlaps(first, second))
        .filter(|b| *b)
        .count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day4t.txt").unwrap();
        assert_eq!(result, 2)
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day4t.txt").unwrap();
        assert_eq!(result, 4)
    }
}
