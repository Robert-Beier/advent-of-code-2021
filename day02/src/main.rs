use day02::{execute_commands, execute_commands_using_aim, parse_commands};
use utils::{read_input, solve};

fn main() {
    let lines = read_input().lines().map(|line| line.to_string()).collect();
    let commands = parse_commands(&lines);

    solve("Part one", || {
        let position = execute_commands(&commands);
        position.x * position.y
    });
    solve("Part two", || {
        let position = execute_commands_using_aim(&commands);
        position.x * position.y
    })
}
