use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::utils;
use thiserror::Error;

#[derive(Error, Debug)]
enum AsciiError {
    #[error("Not ASCII symbol")]
    NotAsciiSymbol(char),

    #[error("Not a letter")]
    NotALetter(char),
}

fn char_to_priority(c: char) -> Result<u8, AsciiError> {
    if !c.is_ascii() {
        return Err(AsciiError::NotAsciiSymbol(c));
    }
    if !c.is_alphabetic() {
        return Err(AsciiError::NotALetter(c));
    }
    let ascii_code = c as u8;
    if ascii_code >= 65 && ascii_code <= 90 {
        return Ok(ascii_code - 38);
    } else if ascii_code >= 97 && ascii_code <= 122 {
        return Ok(ascii_code - 96);
    }
    Ok(0)
}

fn get_rucksak_priority(rucksack: &str) -> Option<u64> {
    let string_len = rucksack.len();
    let chars_slice = rucksack.chars().collect::<Vec<char>>();
    let (first, second) = chars_slice.split_at(string_len / 2);
    let first = BTreeSet::from_iter(first.to_vec().iter().cloned());
    let second = BTreeSet::from_iter(second.to_vec().iter().cloned());
    let intersection = &first & &second;
    Some(
        intersection
            .into_iter()
            .map(|c| char_to_priority(c).unwrap() as u64)
            .sum(),
    )
}

pub fn task_a<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    Ok(utils::get_input_file(file)?
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| get_rucksak_priority(&line))
        .sum())
}

pub fn task_b<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    Ok(utils::get_input_file(file)?
        .lines()
        // .par_bridge()
        .filter_map(|line| line.ok())
        .tuples::<(_, _, _)>()
        .map(|tuple| {
            (
                BTreeSet::from_iter(tuple.0.chars()),
                BTreeSet::from_iter(tuple.1.chars()),
                BTreeSet::from_iter(tuple.2.chars()),
            )
        })
        .map(|group| &(&group.0 & &group.1) & &group.2)
        .map(|badges| {
            badges
                .into_iter()
                .map(|c| char_to_priority(c).unwrap() as u64)
                .sum::<u64>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day3t.txt").unwrap();
        assert_eq!(result, 157)
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day3t.txt").unwrap();
        assert_eq!(result, 70)
    }
}
