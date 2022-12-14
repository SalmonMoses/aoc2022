#![allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day_template;
mod parsers;
mod utils;

extern crate rayon;

fn main() {
    use day6::{task_a, task_b};

    let day_file_path = "data/day6.txt";
    println!("{}", task_a(day_file_path).unwrap());
    println!("{}", task_b(day_file_path).unwrap());
}
