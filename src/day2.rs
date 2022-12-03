use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use rayon::prelude::*;
use thiserror::Error;

use crate::utils;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Unknown symbol {0}")]
    UnknownSymbol(String),

    #[error("Invalid format string {0}")]
    InvalidFormat(String),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_winning_symbol(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn get_losing_symbol(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Shape::Rock),
            "Y" | "B" => Ok(Shape::Paper),
            "Z" | "C" => Ok(Shape::Scissors),
            _ => Err(ParsingError::UnknownSymbol(s.to_string())),
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Lose,
}

impl FromStr for GameResult {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParsingError::UnknownSymbol(s.to_string())),
        }
    }
}

struct Game {
    player: Shape,
    enemy: Shape,
}

impl Game {
    fn get_player_result(&self) -> GameResult {
        if self.player == self.enemy.get_winning_symbol() {
            GameResult::Win
        } else if self.player == self.enemy.get_losing_symbol() {
            GameResult::Lose
        } else {
            GameResult::Draw
        }
    }

    fn get_player_score(&self) -> u32 {
        let outcome_score = match self.get_player_result() {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        };
        self.player.get_score() + outcome_score
    }
}

impl FromStr for Game {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_string = s.split(" ");
        let opponent_shape = splitted_string
            .next()
            .ok_or(ParsingError::InvalidFormat(s.to_string()))
            .and_then(|s| Shape::from_str(s))?;
        let player_shape = splitted_string
            .next()
            .ok_or(ParsingError::InvalidFormat(s.to_string()))
            .and_then(|s| Shape::from_str(s))?;
        Ok(Game {
            player: player_shape,
            enemy: opponent_shape,
        })
    }
}

fn task_b_from_str(s: &str) -> Result<Game, ParsingError> {
    let mut splitted_string = s.split(" ");
    let opponent_shape = splitted_string
        .next()
        .ok_or(ParsingError::InvalidFormat(s.to_string()))
        .and_then(|s| Shape::from_str(s))?;
    let game_result = splitted_string
        .next()
        .ok_or(ParsingError::InvalidFormat(s.to_string()))
        .and_then(|s| GameResult::from_str(s))?;
    let player_shape = match game_result {
        GameResult::Win => opponent_shape.get_winning_symbol(),
        GameResult::Draw => opponent_shape,
        GameResult::Lose => opponent_shape.get_losing_symbol(),
    };
    Ok(Game {
        player: player_shape,
        enemy: opponent_shape,
    })
}

pub fn task_a<P>(file: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    Ok(utils::get_input_file(file)?
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| Game::from_str(&line).ok())
        .map(|game| game.get_player_score())
        .sum())
}

pub fn task_b<P>(file: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    Ok(utils::get_input_file(file)?
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| task_b_from_str(&line).ok())
        .map(|game| game.get_player_score())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_works() {
        let result = task_a("data/day2t.txt").unwrap();
        assert_eq!(result, 15)
    }

    #[test]
    fn test_b_works() {
        let result = task_b("data/day2t.txt").unwrap();
        assert_eq!(result, 12)
    }
}
