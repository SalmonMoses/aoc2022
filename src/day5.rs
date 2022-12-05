use crate::parsers::integer;
use crate::utils;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, multispace0, space0};
use nom::combinator::{map_res, opt};
use nom::error::dbg_dmp;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

type CrateStack<'a> = VecDeque<Crate<'a>>;
type Stacks<'a> = Vec<RefCell<CrateStack<'a>>>;

struct Movement {
    quantity: u64,
    from: u64,
    to: u64,
}

impl Movement {
    fn new(quantity: u64, from: u64, to: u64) -> Self {
        Self { quantity, from, to }
    }

    fn execute(&self, stacks: &Stacks) {
        let mut from_stack = stacks.get((self.from - 1) as usize).unwrap().borrow_mut();
        let mut to_stack = stacks.get((self.to - 1) as usize).unwrap().borrow_mut();
        for i in 0..self.quantity {
            let moved_crate = from_stack.pop_front().unwrap();
            to_stack.push_front(moved_crate);
        }
    }

    fn execute_task_b(&self, stacks: &Stacks) {
        let mut from_stack = stacks.get((self.from - 1) as usize).unwrap().borrow_mut();
        let mut temp = VecDeque::<Crate>::with_capacity(self.quantity as usize);
        for i in 0..self.quantity {
            let moved_crate = from_stack.pop_front().unwrap();
            temp.push_back(moved_crate);
        }
        let mut to_stack = stacks.get((self.to - 1) as usize).unwrap().borrow_mut();
        for i in 0..self.quantity {
            let moved_crate = temp.pop_back().unwrap();
            to_stack.push_front(moved_crate);
        }
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} ({})", self.from, self.to, self.quantity)
    }
}

struct Crate<'a> {
    name: &'a str,
}

fn parse_all_cmds(input: &str) -> nom::IResult<&str, Vec<Movement>> {
    separated_list1(line_ending, parse_movement_command)(input)
}

fn parse_movement_command(input: &str) -> nom::IResult<&str, Movement> {
    nom::combinator::map(
        tuple((
            tag("move "),
            integer,
            tag(" from "),
            integer,
            tag(" to "),
            integer,
        )),
        |(_, quantity, _, from, _, to)| Movement::new(quantity, from, to),
    )(input)
}

fn parse_stacks_number(input: &str) -> Option<u64> {
    input
        .split(" ")
        .map(|int| u64::from_str(int).unwrap_or(0))
        .max()
}

fn parse_empty_crate(input: &str) -> nom::IResult<&str, Option<Crate>> {
    nom::combinator::map(tag("   "), |_| None)(input)
}

fn parse_crate(input: &str) -> nom::IResult<&str, Option<Crate>> {
    nom::combinator::map(delimited(tag("["), alpha1, tag("]")), |name| {
        Some(Crate { name })
    })(input)
}

fn parse_crate_line(input: &str) -> nom::IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), alt((parse_crate, parse_empty_crate)))(input)
}

pub fn task_a<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    let mut input_lines = input.split("\n\n");
    let crates = input_lines.next().unwrap();
    let mut crates_iter = crates.lines().rev();
    let stacks_line = crates_iter.next().unwrap();
    let crates_num = parse_stacks_number(stacks_line).unwrap();
    let mut stacks = (0..crates_num)
        .map(|_| RefCell::new(CrateStack::new()))
        .collect::<Vec<_>>();
    for stack_line in crates_iter {
        let crates = parse_crate_line(stack_line).unwrap().1;
        crates
            .into_iter()
            .enumerate()
            .filter(|maybe_crate| maybe_crate.1.is_some())
            .for_each(|(i, cur_crate)| stacks[i].borrow_mut().push_front(cur_crate.unwrap()));
    }
    let commands = input_lines.next().unwrap();
    let parsed_commands = parse_all_cmds(commands).unwrap().1;
    parsed_commands.iter().for_each(|cmd| cmd.execute(&stacks));
    Ok(stacks
        .iter()
        .map(|stack| stack.borrow().front().unwrap().name)
        .join(""))
}

pub fn task_b<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let input = utils::get_input_string(file)?;
    let mut input_lines = input.split("\n\n");
    let crates = input_lines.next().unwrap();
    let mut crates_iter = crates.lines().rev();
    let stacks_line = crates_iter.next().unwrap();
    let crates_num = parse_stacks_number(stacks_line).unwrap();
    let mut stacks = (0..crates_num)
        .map(|_| RefCell::new(CrateStack::new()))
        .collect::<Vec<_>>();
    for stack_line in crates_iter {
        let crates = parse_crate_line(stack_line).unwrap().1;
        crates
            .into_iter()
            .enumerate()
            .filter(|maybe_crate| maybe_crate.1.is_some())
            .for_each(|(i, cur_crate)| stacks[i].borrow_mut().push_front(cur_crate.unwrap()));
    }
    let commands = input_lines.next().unwrap();
    let parsed_commands = parse_all_cmds(commands).unwrap().1;
    parsed_commands
        .iter()
        .for_each(|cmd| cmd.execute_task_b(&stacks));
    Ok(stacks
        .iter()
        .map(|stack| stack.borrow().front().unwrap().name)
        .join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day5t.txt").unwrap();
        assert_eq!(result, "CMZ")
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day5t.txt").unwrap();
        assert_eq!(result, "MCD")
    }
}
