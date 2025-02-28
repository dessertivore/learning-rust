mod problems;
use either;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_number: i64 = args[1]
        .parse::<i64>()
        .expect("Failed to parse input as i64");

    let cli_problem_input = args[2].parse::<i64>();
    let problem_input: Option<i64> = match cli_problem_input {
        Ok(value) => Some(value),
        Err(_) => None,
    };

    println!("Solving problem number: {problem_number}");
    println!("Using problem input: {:?}", problem_input);

    let result: either::Either<&str, i64> = match problem_number {
        1 => either::Right(problems::problem_1::problem_1(
            problem_input.unwrap_or(10000),
        )),
        2 => either::Right(problems::problem_2::problem_2(
            problem_input.unwrap_or(4000000),
        )),
        3 => either::Right(problems::problem_3::problem_3(
            problem_input.unwrap_or(600851475143),
        )),
        _ => either::Left("Solution not written yet"),
    };
    println!("{}", result)
}
