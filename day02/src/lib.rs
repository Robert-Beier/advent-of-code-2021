#[derive(PartialEq, Debug)]
pub enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Direction {
    fn from_string(direction: &str) -> Direction {
        match direction {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            "up" => Direction::UP,
            _ => panic!("Direction could not be parsed."),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Command {
    direction: Direction,
    steps: u32,
}

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn parse_commands(commands: &Vec<String>) -> Vec<Command> {
    commands
        .into_iter()
        .map(|command| {
            let mut parts = command.split(' ');
            Command {
                direction: Direction::from_string(
                    parts.next().expect("Direction could not be parsed."),
                ),
                steps: parts
                    .next()
                    .expect("Steps could not be parsed.")
                    .parse()
                    .expect("Steps could not be parsed."),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests_parse_commands {
    use crate::{parse_commands, Command, Direction};

    #[test]
    fn should_work_for_example_one() {
        let commands = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .into_iter()
        .map(|command| command.to_string())
        .collect();

        let expected_result = vec![
            Command {
                direction: Direction::FORWARD,
                steps: 5,
            },
            Command {
                direction: Direction::DOWN,
                steps: 5,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 8,
            },
            Command {
                direction: Direction::UP,
                steps: 3,
            },
            Command {
                direction: Direction::DOWN,
                steps: 8,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 2,
            },
        ];

        assert_eq!(parse_commands(&commands), expected_result);
    }
}

pub fn execute_commands(commands: &Vec<Command>) -> Point {
    let mut position = Point { x: 0, y: 0 };

    commands.into_iter().for_each(|command| {
        match command.direction {
            Direction::FORWARD => position.x += command.steps,
            Direction::DOWN => position.y += command.steps,
            // we are relying only on the rust debug mode to check underflow here
            Direction::UP => position.y -= command.steps,
        };
    });

    position
}

#[cfg(test)]
mod tests_execute_commands {
    use crate::{execute_commands, Command, Direction, Point};

    #[test]
    fn should_work_for_example_one() {
        let commands = vec![
            Command {
                direction: Direction::FORWARD,
                steps: 5,
            },
            Command {
                direction: Direction::DOWN,
                steps: 5,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 8,
            },
            Command {
                direction: Direction::UP,
                steps: 3,
            },
            Command {
                direction: Direction::DOWN,
                steps: 8,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 2,
            },
        ];

        assert_eq!(execute_commands(&commands), Point { x: 15, y: 10 });
    }
}

pub fn execute_commands_using_aim(commands: &Vec<Command>) -> Point {
    let mut position = Point { x: 0, y: 0 };
    let mut aim: u32 = 0;

    commands.into_iter().for_each(|command| {
        match command.direction {
            Direction::FORWARD => {
                position.x += command.steps;
                position.y += command.steps * aim;
            }
            Direction::DOWN => aim += command.steps,
            // we are relying only on the rust debug mode to check underflow here
            Direction::UP => aim -= command.steps,
        };
    });

    position
}

#[cfg(test)]
mod tests_execute_commands_using_aim {
    use crate::{execute_commands_using_aim, Command, Direction, Point};

    #[test]
    fn should_work_for_example_one() {
        let commands = vec![
            Command {
                direction: Direction::FORWARD,
                steps: 5,
            },
            Command {
                direction: Direction::DOWN,
                steps: 5,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 8,
            },
            Command {
                direction: Direction::UP,
                steps: 3,
            },
            Command {
                direction: Direction::DOWN,
                steps: 8,
            },
            Command {
                direction: Direction::FORWARD,
                steps: 2,
            },
        ];

        assert_eq!(
            execute_commands_using_aim(&commands),
            Point { x: 15, y: 60 }
        );
    }
}
