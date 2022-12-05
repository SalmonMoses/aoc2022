use nom::character::complete::digit1;
use nom::combinator::map_res;
use std::str::FromStr;

pub fn integer(input: &str) -> nom::IResult<&str, u64> {
    map_res(digit1, u64::from_str)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_parses() {
        let result = integer("64").unwrap().1;
        assert_eq!(result, 64)
    }
}
