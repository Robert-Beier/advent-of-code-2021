use day02::{execute_commands, parse_commands};
use utils::{read_input, solve};

fn main() {
    let lines = read_input().lines().map(|line| line.to_string()).collect();
    let commands = parse_commands(&lines);

    solve("Part one", || {
        let position = execute_commands(&commands);
        position.x * position.y
    })
}
