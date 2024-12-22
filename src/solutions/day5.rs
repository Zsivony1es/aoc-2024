use std::collections::HashMap;
use itertools::Itertools;

use super::Solve;

pub struct Day5 {
    pub input: String
}

impl Solve for Day5 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        let mut parts = self.input.split("\n\n").collect::<Vec<&str>>();
        let updates = get_update_list(parts.pop().unwrap());
        let ruleset     = get_ruleset(parts.pop().unwrap());

        let mut sum: u32 = 0;

        for update in updates.iter() {

            if is_valid(update, &ruleset){
                let middle_value = u32::from(update[(update.len()) / 2]);
                sum += middle_value;
            }
        }
        
        Ok(sum.to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        let mut parts = self.input.split("\n\n").collect::<Vec<&str>>();
        let updates = get_update_list(parts.pop().unwrap());
        let ruleset     = get_ruleset(parts.pop().unwrap());

        let mut sum: u32 = 0;

        for update in updates.iter() {
            if !is_valid(update, &ruleset){
                let reordered_updates = reorder_with_ruleset(update, &ruleset);
                let middle_value = u32::from(reordered_updates[(reordered_updates.len()) / 2]);
                sum += middle_value;
            }
        }

        Ok(sum.to_string())
    }
}

fn reorder_with_ruleset(row: &Vec<u8>, ruleset: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut first: bool = true;
    for page in row.iter() {
        if first {
            result.push(*page);
            first = false;
            continue;
        }
        let mut inserted = false;
        for i in 0..result.len() {
            match ruleset.get(page) {
                Some(v) => {
                    if v.contains(&result[i]) {
                        inserted = true;
                        result.insert(i, *page);
                        break;
                    }
                },
                None => continue
            };
        }
        if !inserted {
            result.push(*page);
        }
    }
    result
}

fn is_valid(row: &Vec<u8>, ruleset: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 0..row.len() {
        match ruleset.get(&row[i]) {
            Some(v) => {
                for j in 0..v.len() {
                    for k in 0..i {
                        if v[j] == row[k] {
                            return false;
                        }
                    }
                }
            },
            None => continue
        };
    }
    true
}

fn get_ruleset(input: &str) -> HashMap<u8, Vec<u8>> {
    input
        .split("\n")
        .map(|s| s.split("|").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>())
        .into_group_map_by(|v| v[0])
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(|pair| pair[1]).collect()))
        .collect::<HashMap<u8, Vec<u8>>>()
}

fn get_update_list(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|row| row
                        .split(',')
                        .map(|num| num.parse::<u8>().unwrap())
                        .collect()
            )
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
        let day = Day5 { input };
        assert_eq!(day.solve_task_one().unwrap(), "143");
    }

    #[test]
    fn test_task_two() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
        let day = Day5 { input };
        assert_eq!(day.solve_task_two().unwrap(), "123");
    }

}