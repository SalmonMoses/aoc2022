use std::fs::File;
use std::{io, mem};
use std::io::{BufRead, BufReader};
use rayon::prelude::*;

pub fn get_elves(input: BufReader<File>) -> Vec<Vec<u64>> {
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

pub fn task_a() -> io::Result<u64> {
    let input = File::open("data/day1.txt")?;
    let buf_reader = io::BufReader::new(input);
    let elves = get_elves(buf_reader);
    Ok(elves.par_iter()
        .map(|elf| elf.into_iter().sum::<u64>())
        .max()
        .unwrap_or(0u64))
}

pub fn task_b() -> io::Result<u64> {
    let input = File::open("data/day1.txt")?;
    let buf_reader = io::BufReader::new(input);
    let elves = get_elves(buf_reader);
    let mut sorted_elves = elves
        .par_iter()
        .map(|elf| elf.into_iter().sum::<u64>())
        .collect::<Vec<u64>>();
    sorted_elves.par_sort();
    Ok(sorted_elves
        .par_iter()
        .rev()
        .take(3)
        .sum())
}