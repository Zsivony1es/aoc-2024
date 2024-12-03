mod day1;
mod day2;
mod day3;
mod day4;

pub trait Solve {
    fn solve_task_one(&self) -> Result<String, &'static str>;
    fn solve_task_two(&self) -> Result<String, &'static str>;
}
fn read_input(day: &u8) -> String {
    let path = format!("res/input/day{}.txt", day);
    std::fs::read_to_string(path).unwrap()
}

pub fn get_solver(day: &u8) -> Box<dyn Solve> {
    match day {
        1 => Box::new(day1::Day1{ day: day.clone(), input: read_input(day) }),
        2 => Box::new(day2::Day2{ day: day.clone(), input: read_input(day) }),
        3 => Box::new(day3::Day3{ day: day.clone(), input: read_input(day) }),
        _ => panic!("No solver for day {}", day),
    }
}