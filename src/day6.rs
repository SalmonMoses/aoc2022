use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

pub fn task_a<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    unimplemented!()
}

pub fn task_b<P>(file: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day6t.txt").unwrap();
        assert_eq!(result, "")
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day6t.txt").unwrap();
        assert_eq!(result, "")
    }
}
