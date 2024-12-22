mod solutions;

use chrono;

fn main() {
    let today: String = chrono::Local::now().format("%d").to_string();
    let today: u8 = today.parse().unwrap();
    let today = 5;
    println!("Solving today's tasks: Day {}", today);

    let solver = solutions::get_solver(&today);
    println!("Task 1 solution: {}", solver.solve_task_one().unwrap());
    println!("Task 2 solution: {}", solver.solve_task_two().unwrap());

}
