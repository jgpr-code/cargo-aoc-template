use anyhow::Result;
use std::io::{self, Read};

struct Input {}

fn parse_buffer(buffer: &str) -> Result<Input> {
    Ok(Input {})
}

fn part_one(input: &Input) -> Result<i128> {
    Ok(0)
}

fn part_two(input: &Input) -> Result<i128> {
    Ok(0)
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = parse_buffer(&buffer)?;
    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref TEST: Input = read_from_file("test.txt");
        static ref INPUT: Input = read_from_file("input.txt");
    }

    fn read_from_file(filename: &str) -> Input {
        let buffer = fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg));

        parse_buffer(&buffer).unwrap_or_else(|msg| panic!("error parsing {}: {}", filename, msg))
    }

    #[test]
    fn part_one_on_test() -> Result<()> {
        let answer = part_one(&TEST)?;
        assert_eq!(answer, 0);
        Ok(())
    }
    #[test]
    fn part_one_on_input() -> Result<()> {
        let answer = part_one(&INPUT)?;
        assert_eq!(answer, 0);
        Ok(())
    }
    #[test]
    fn part_two_on_test() -> Result<()> {
        let answer = part_two(&TEST)?;
        assert_eq!(answer, 0);
        Ok(())
    }
    #[test]
    fn part_two_on_input() -> Result<()> {
        let answer = part_two(&INPUT)?;
        assert_eq!(answer, 0);
        Ok(())
    }
}
