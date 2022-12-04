use crate::utils;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_parser, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use rayon::prelude::*;
use std::convert::identity;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

use thiserror::Error;

type Assignment = (RangeInclusive<u64>, RangeInclusive<u64>);

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Something is wrong")]
    Err,
}

fn parse_file(input: &str) -> nom::IResult<&str, Vec<Assignment>> {
    separated_list1(tag("\r\n"), parse_line)(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, Assignment> {
    nom::combinator::map(
        tuple((parse_group, tag(","), parse_group)),
        |(first, _, second)| (first, second),
    )(input)
}

fn parse_group(input: &str) -> nom::IResult<&str, RangeInclusive<u64>> {
    nom::combinator::map(tuple((integer, tag("-"), integer)), |(start, _, end)| {
        start..=end
    })(input)
}

fn integer(input: &str) -> nom::IResult<&str, u64> {
    map_res(digit1, u64::from_str)(input)
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
