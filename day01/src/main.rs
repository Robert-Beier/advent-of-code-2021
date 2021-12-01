use day01::{count_number_of_increments, count_number_of_increments_using_three_measurement_sliding_window};
use utils::{read_input, solve};

fn main() {
    let steps: Vec<u32> = read_input()
        .lines()
        .map(|n| n.parse::<u32>().expect("Failed to parse number."))
        .collect();
    solve("Part one", || count_number_of_increments(&steps));
    solve("Part two", || count_number_of_increments_using_three_measurement_sliding_window(&steps));
}
