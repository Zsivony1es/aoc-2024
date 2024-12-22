mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

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
        1 => Box::new(day1::Day1{ input: read_input(day) }),
        2 => Box::new(day2::Day2{ input: read_input(day) }),
        3 => Box::new(day3::Day3{ input: read_input(day) }),
        4 => Box::new(day4::Day4{ input: read_input(day) }),
        5 => Box::new(day5::Day5{ input: read_input(day) }),
        6 => Box::new(day6::Day6{ input: read_input(day) }),
        7 => Box::new(day7::Day7{ input: read_input(day) }),
        8 => Box::new(day8::Day8{ input: read_input(day) }),
        9 => Box::new(day9::Day9{ input: read_input(day) }),
        10 => Box::new(day10::Day10{ input: read_input(day) }),
        11 => Box::new(day11::Day11{ input: read_input(day) }),
        12 => Box::new(day12::Day12{ input: read_input(day) }),
        13 => Box::new(day13::Day13{ input: read_input(day) }),
        14 => Box::new(day14::Day14{ input: read_input(day) }),
        15 => Box::new(day15::Day15{ input: read_input(day) }),
        16 => Box::new(day16::Day16{ input: read_input(day) }),
        17 => Box::new(day17::Day17{ input: read_input(day) }),
        18 => Box::new(day18::Day18{ input: read_input(day) }),
        19 => Box::new(day19::Day19{ input: read_input(day) }),
        20 => Box::new(day20::Day20{ input: read_input(day) }),
        21 => Box::new(day21::Day21{ input: read_input(day) }),
        22 => Box::new(day22::Day22{ input: read_input(day) }),
        23 => Box::new(day23::Day23{ input: read_input(day) }),
        24 => Box::new(day24::Day24{ input: read_input(day) }),
        _ => panic!("No solver for day {}", day),
    }
}