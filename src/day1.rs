use crate::utils;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{io, mem};

fn get_elves(input: BufReader<File>) -> Vec<Vec<u64>> {
    let mut elves: Vec<Vec<u64>> = vec![];
    let mut cur_elf: Vec<u64> = vec![];
    for line in input.lines() {
        if let Ok(line) = line {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                elves.push(mem::take(&mut cur_elf));
            } else {
                let calories = line.parse::<u64>().unwrap();
                cur_elf.push(calories);
            }
        }
    }
    elves.push(mem::take(&mut cur_elf));
    elves
}

pub fn task_a<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    let input_reader = utils::get_input_file(file)?;
    let elves = get_elves(input_reader);
    Ok(elves
        .par_iter()
        .map(|elf| elf.into_iter().sum::<u64>())
        .max()
        .unwrap_or(0u64))
}

pub fn task_b<P>(file: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    let input_reader = utils::get_input_file(file)?;
    let elves = get_elves(input_reader);
    let mut sorted_elves = elves
        .par_iter()
        .map(|elf| elf.into_iter().sum::<u64>())
        .collect::<Vec<u64>>();
    sorted_elves.par_sort();
    Ok(sorted_elves.par_iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day1t.txt");
        assert_eq!(result.unwrap(), 24000);
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day1t.txt");
        assert_eq!(result.unwrap(), 45000);
    }
}
