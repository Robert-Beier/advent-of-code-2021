pub fn count_number_of_increments(steps: &Vec<u32>) -> usize {
    steps.into_iter().enumerate().filter(|(index, value)| {
        *index != 0 && *value > steps.get(index - 1).expect("Previous step not found.")
    }).count()
}

#[cfg(test)]
mod tests_count_number_of_increments {
    use crate::count_number_of_increments;

    #[test]
    fn should_count_increments_but_not_the_first_step() {
        let steps = vec![199, 200, 208, 210];
        assert_eq!(count_number_of_increments(&steps), 3);
    }

    #[test]
    fn should_not_count_decrements_as_increments() {
        let steps = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_number_of_increments(&steps), 7);
    }

    #[test]
    fn should_not_count_equal_consecutive_steps_as_increments() {
        let steps = vec![199, 200, 208, 210, 210, 211, 240, 269, 269, 270];
        assert_eq!(count_number_of_increments(&steps), 7);
    }

}