use super::Solve;

use std::collections::HashMap;

pub struct Day7 {
    pub input: String
}

impl Solve for Day7 {
    fn solve_task_one(&self) -> Result<String, &'static str> {

        let input: HashMap<u64, Vec<u32>> = self.input
            .split('\n')
            .map(|s| {
                let s = s.split(": ").collect::<Vec<&str>>();
                (s[0].parse::<u64>().unwrap(), s[1].split(' ').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            })
            .collect();

        let mut result: u64 = 0;
        
        for (k, v) in input.iter() {
            let current_sum: u64 = v[0] as u64;
            if recurse(current_sum, v, 1, k) {
                result += k;
            }
        }

        Ok(result.to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        let input: HashMap<u64, Vec<u32>> = self.input
            .split('\n')
            .map(|s| {
                let s = s.split(": ").collect::<Vec<&str>>();
                (s[0].parse::<u64>().unwrap(), s[1].split(' ').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            })
            .collect();

        let mut result: u64 = 0;
        
        for (k, v) in input.iter() {
            let current_sum: u64 = v[0] as u64;
            if recurse_pt2(current_sum, v, 1, k) {
                result += k;
            }
        }

        Ok(result.to_string())
    }
}

fn recurse(current_sum: u64, numbers: &Vec<u32>, depth: usize, target_sum: &u64) -> bool {
    if current_sum as u64 == *target_sum && depth == numbers.len() { return true; } else if depth == numbers.len() { return false; }
    let vec = vec![current_sum + numbers[depth] as u64, 
                            current_sum * numbers[depth] as u64];
    vec.iter().any(|v| recurse(*v, numbers, depth + 1, target_sum))
}

fn recurse_pt2(current_sum: u64, numbers: &Vec<u32>, depth: usize, target_sum: &u64) -> bool {
    if current_sum as u64 == *target_sum && depth == numbers.len() { return true; } else if depth == numbers.len() { return false; }
    let concatted = current_sum.to_string() + numbers[depth].to_string().as_str();
    let vec = vec![current_sum + numbers[depth] as u64, 
                            current_sum * numbers[depth] as u64, 
                            concatted.parse::<u64>().unwrap()];
    vec.iter().any(|v| recurse_pt2(*v, numbers, depth + 1, target_sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20".to_string();
        let day = Day7 { input };
        assert_eq!(day.solve_task_one().unwrap(), "3749");
    }

    #[test]
    fn test_task_two() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20".to_string();
        let day = Day7 { input };
        assert_eq!(day.solve_task_two().unwrap(), "11387");
    }

}