use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

pub fn task_a<P>(file: P) -> io::Result<u64> where P: AsRef<Path> {
    unimplemented!()
}

pub fn task_b<P>(file: P) -> io::Result<u64> where P: AsRef<Path> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::day_template::{task_a, task_b};

    #[test]
    fn test_a_works() {
        assert!(false)
    }

    #[test]
    fn test_b_works() {
        assert!(false)
    }
}