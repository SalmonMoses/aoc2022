mod task1;

extern crate rayon;

fn main() {
    println!("{}", task1::task_a().unwrap());
    println!("{}", task1::task_b().unwrap());
}
