use std::collections::HashMap;

use super::Solve;

pub struct Day1 {
    pub input: String
}

impl Solve for Day1 {

    fn solve_task_one(&self) -> Result<String, &'static str> {
        let mut vec1: Vec<i32> = Vec::new();
        let mut vec2: Vec<i32> = Vec::new();

        for line in self.input.lines() {
            let nums: Vec<i32> = line.split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
            vec1.push(nums[0]);
            vec2.push(nums[1]);
        }

        vec1.sort();
        vec2.sort();
        let mut sum: i32 = 0;

        for i in 0..vec1.len() {
            sum += (vec1[i] - vec2[i]).abs();
        }

        Ok(sum.to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec1: Vec<i32> = Vec::new();

        for line in self.input.lines() {
            let nums: Vec<i32> = line.split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
            vec1.push(nums[0]);
            if map.contains_key(&nums[1]) {
                map.insert(nums[1], map.get(&nums[1]).unwrap() + 1);
            } else {
                map.insert(nums[1], 1);
            }
        }

        let result = vec1.iter().fold(0, |acc, x| {
            if map.contains_key(x) {
                acc + map.get(x).unwrap() * x
            } else {
                acc
            }
        } );

        Ok(result.to_string())
    }
}
