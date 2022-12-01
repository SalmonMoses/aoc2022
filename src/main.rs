mod day1;
mod day_template;

extern crate rayon;

fn main() {
    println!("{}", day1::task_a("data/day1.txt").unwrap());
    println!("{}", day1::task_b("data/day1.txt").unwrap());
}
