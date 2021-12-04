use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {

    type Err = String;
    
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut data = data.trim().split_ascii_whitespace();
        let direction = data.next().ok_or(String::from("No direction"))?;
        let distance = data.next().ok_or(String::from("No distance"))?;
        let distance: usize = distance.parse().map_err(|_| String::from("Invalid distance"))?;
        match direction {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            _  => Err(String::from("Invalid direction"))
        }
    }
}

pub struct SubmarinePosition {
    pub horizontal_position: usize,
    pub depth: usize,

}

pub fn calculate_submarine_position(commands: &Vec<Command>) -> SubmarinePosition {
    let (horizontal_position, depth) = commands.iter().fold((0, 0), |(hp, depth), command| {
        match command {
            Command::Forward(v) => (hp + v, depth),
            Command::Down(v) => (hp, depth + v),
            Command::Up(v) => (hp, depth - v),
        }
    });
    SubmarinePosition { horizontal_position, depth }
}

pub fn calculate_submarine_position_with_aim(commands: &Vec<Command>) -> SubmarinePosition {
    let (horizontal_position, depth, _) = commands.iter().fold((0, 0, 0), |(hp, depth, aim), command| {
        match command {
            Command::Forward(v) => (hp + v, depth + (aim * v), aim),
            Command::Down(v) => (hp, depth, aim + v),
            Command::Up(v) => (hp, depth, aim - v),
        }
    });
    SubmarinePosition { horizontal_position, depth }
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn day_2_part_1_solution() {
        let commands: Vec<Command> = data::read_input_data("./data/d02/data.txt").expect("Invalid input data");
        let submarine_position = calculate_submarine_position(&commands);
        let position_multiplication = submarine_position.horizontal_position * submarine_position.depth;
        assert_eq!(position_multiplication, 2070300);
    }

    #[test]
    fn day_2_part_2_solution() {
        let commands: Vec<Command> = data::read_input_data("./data/d02/data.txt").expect("Invalid input data");
        let submarine_position = calculate_submarine_position_with_aim(&commands);
        let position_multiplication = submarine_position.horizontal_position * submarine_position.depth;
        assert_eq!(position_multiplication, 2078985210);
    }
}