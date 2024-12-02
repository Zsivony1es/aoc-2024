use super::Solve;

pub struct Day2 {
    pub day: u8,
    pub input: String
}

impl Solve for Day2 {

    fn solve_task_one(&self) -> Result<String, &'static str> {
        let safe_count = self.input.lines()
        .filter(|line | {
            let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
            safe_without_dampening(&nums)
        })
        .count();

        Ok(safe_count.to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        let safe_count = self.input.lines()
        .filter(|line | {
            let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
            safe_with_dampening(&nums)
        })
        .count();

        Ok(safe_count.to_string())
    }
}

pub fn safe_without_dampening(nums: &Vec<i32>) -> bool {
        let is_increasing = nums.windows(2).all(|w| w[1] > w[0]);
        let is_decreasing = nums.windows(2).all(|w| w[1] < w[0]);
        
        let valid_differences = nums.windows(2)
            .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3);
        
        (is_increasing || is_decreasing) && valid_differences
}

pub fn safe_with_dampening(nums: &Vec<i32>) -> bool {
    if safe_without_dampening(nums) {
        true
    } else {
        for i in 0..nums.len(){
            let mut v = nums.clone();
            v.remove(i);
            if safe_without_dampening(&v) {
                return true;
            }
        }
        false
    }
}