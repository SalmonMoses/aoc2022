#![allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day_template;
mod utils;

extern crate rayon;

fn main() {
    use day4::{task_a, task_b};

    let day_file_path = "data/day4.txt";
    println!("{}", task_a(day_file_path).unwrap());
    println!("{}", task_b(day_file_path).unwrap());
}
