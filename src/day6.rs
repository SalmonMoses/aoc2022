use crate::utils;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

fn find_marker(signal: &str, unique_len: usize) -> usize {
    for i in signal.char_indices().skip(unique_len) {
        let substring = &signal[(i.0 - unique_len)..i.0];
        let substring_chars = substring.chars().collect::<BTreeSet<_>>();
        if substring.len() == substring_chars.len() {
            return i.0;
        }
    }
    0
}

pub fn task_a<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    Ok(find_marker(&input, 4).to_string())
}

pub fn task_b<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    Ok(find_marker(&input, 14).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_marker_works() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }

    #[test]
    fn test_a_works() {
        // let result = task_a("data/day6t.txt").unwrap();
        // assert_eq!(result, "")
    }

    #[test]
    fn test_b_works() {
        // let result = task_b("data/day6t.txt").unwrap();
        // assert_eq!(result, "")
    }
}
