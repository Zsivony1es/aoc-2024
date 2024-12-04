use super::Solve;

use regex::Regex;

pub struct Day3 {
    pub input: String
}

impl Solve for Day3 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let result = re
            .find_iter(&self.input)
            .fold(0, |acc, ma| {
                let mystr: &str = ma.as_str();
                let mystr: &str = &mystr[4..mystr.len() - 1];
                acc + mystr
                    .split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .fold(1, |ac, x| ac * x)
            });
        Ok(result.to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        let mul_re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let do_re: Regex = Regex::new(r"do(n't)?\(\)").unwrap();

        let mut do_starts: Vec<usize> = do_re
            .find_iter(&self.input)
            .filter(|m| !m.as_str().contains("don't"))
            .map(|m| m.start())
            .collect::<Vec<usize>>();

        do_starts.push(0);

        let mut dont_starts: Vec<usize> = do_re
            .find_iter(&self.input)
            .filter(|m| m.as_str().contains("don't"))
            .map(|m| m.start())
            .collect::<Vec<usize>>();

        do_starts.sort_unstable();
        dont_starts.sort_unstable();

        let result = mul_re
            .find_iter(&self.input)
            .fold(0, |acc, ma| {

                let closest_do_start = do_starts
                    .iter()
                    .rev()
                    .find(|&&x| x <= ma.start())
                    .unwrap();

                let closest_dont_start = dont_starts
                    .iter()
                    .rev()
                    .find(|&&x| x <= ma.start());
                
                if closest_dont_start.is_none() || *closest_dont_start.unwrap() < *closest_do_start {
                    let mystr: &str = ma.as_str();
                    let mystr: &str = &mystr[4..mystr.len() - 1];
                    acc + mystr
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .fold(1, |ac, x| ac * x)
                } else {
                    acc
                }
            });
        Ok(result.to_string())
    }
}
