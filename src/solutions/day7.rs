use super::Solve;

pub struct Day7 {
    pub input: String
}

impl Solve for Day7 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        Ok(String::new())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        let input = "".to_string();
        let day = Day7 { input };
        assert_eq!(day.solve_task_one().unwrap(), "");
    }

    #[test]
    fn test_task_two() {
        let input = "".to_string();
        let day = Day7 { input };
        assert_eq!(day.solve_task_two().unwrap(), "");
    }

}