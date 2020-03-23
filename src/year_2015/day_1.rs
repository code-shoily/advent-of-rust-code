#[allow(dead_code)]
/// Solution of year 2015, day 1. Link: https://adventofcode.com/2015/day/1
pub mod solution {
    use crate::helpers::read_input_file;

    fn process(data: String) -> Vec<char> {
        data.trim().chars().collect()
    }

    #[test]
    fn process_works() {
        let expected_output = vec!['(', '(', ')', ')', '(', '('];
        assert_eq!(process(String::from("(())((")), expected_output);
        assert_eq!(process(String::from("(())((\n")), expected_output);
    }

    fn compute_floor(data: String) -> i32 {
        let commands = process(data);
        let mut total = 0;
        for command in commands.iter() {
            if command.eq_ignore_ascii_case(&'(') {
                total += 1;
            } else {
                total -= 1;
            }
        }
        total
    }

    #[test]
    fn compute_floor_works() {
        assert_eq!(compute_floor(String::from("(())")), 0);
        assert_eq!(compute_floor(String::from("()()")), 0);
        assert_eq!(compute_floor(String::from("(((")), 3);
        assert_eq!(compute_floor(String::from("(()(()(")), 3);
        assert_eq!(compute_floor(String::from("))(((((")), 3);
        assert_eq!(compute_floor(String::from("())")), -1);
        assert_eq!(compute_floor(String::from("))(")), -1);
        assert_eq!(compute_floor(String::from(")))")), -3);
        assert_eq!(compute_floor(String::from(")())())")), -3);
    }

    fn run_1() -> i32 {
        compute_floor(read_input_file(2015, 1))
    }

    #[allow(clippy::explicit_counter_loop)]
    fn steps_to_basement(data: String) -> i32 {
        let commands = process(data);
        let mut total = 0;
        let mut steps = 0;
        for command in commands.iter() {
            if command.eq_ignore_ascii_case(&'(') {
                total += 1;
            } else {
                total -= 1;
            }

            steps += 1;

            if total == -1 {
                return steps;
            }
        }
        panic!("One should never get here")
    }

    #[test]
    fn get_to_basement_works() {
        assert_eq!(steps_to_basement(String::from(")")), 1);
        assert_eq!(steps_to_basement(String::from("()())")), 5);
    }

    fn run_2() -> i32 {
        steps_to_basement(read_input_file(2015, 1))
    }

    pub fn run() -> (i32, i32) {
        (run_1(), run_2())
    }
}
