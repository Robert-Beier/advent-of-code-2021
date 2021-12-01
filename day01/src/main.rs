use day01::count_number_of_increments;
use utils::{read_input, solve};

fn main() {
    let steps: Vec<u32> = read_input()
        .lines()
        .map(|n| n.parse::<u32>().expect("Failed to parse number."))
        .collect();
    solve("Part one", || count_number_of_increments(&steps));
}
