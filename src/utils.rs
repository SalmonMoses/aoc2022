use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

pub fn get_input_file<P>(path: P) -> Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let input_file = File::open(path)?;
    Ok(BufReader::new(input_file))
}

#[allow(dead_code)]
pub fn get_input_string<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut input_file = File::open(path)?;
    let mut result = String::new();
    input_file.read_to_string(&mut result)?;
    Ok(result)
}
