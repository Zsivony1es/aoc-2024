use super::Solve;

pub struct Day6 {
    pub input: String
}

impl Solve for Day6 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        let mut map = get_map(&self.input);

        while move_guard(&mut map) {}

        Ok(get_visited_pos_count(map).to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        Ok(String::new())
    }
}

fn move_guard(input: &mut Vec<Vec<char>>) -> bool {
    let row_count = input.len();
    let col_count = input[0].len();

    for i in 0..row_count {
        for j in 0..col_count {
            match input[i][j] {
                '>' => {
                    if j + 1 >= col_count {
                        input[i][j] = 'X';
                        return false;
                    }
                    if input[i][j+1] == '.' || input[i][j+1] == 'X' {
                        input[i][j] = 'X';
                        input[i][j+1] = '>';
                    }
                    if input[i][j+1] == '#' {
                        input[i][j] = 'v';
                    }
                }
                '<' => {
                    if j == 0 {
                        input[i][j] = 'X';
                        return false;
                    }
                    if input[i][j-1] == '.' || input[i][j-1] == 'X' {
                        input[i][j] = 'X';
                        input[i][j-1] = '<';
                    }
                    if input[i][j-1] == '#' {
                        input[i][j] = '^';
                    }
                }
                'v' => {
                    if i + 1 >= row_count {
                        input[i][j] = 'X';
                        return false;
                    }
                    if input[i+1][j] == '.' || input[i+1][j] == 'X' {
                        input[i][j] = 'X';
                        input[i+1][j] = 'v';
                    }
                    if input[i+1][j] == '#' {
                        input[i][j] = '<';
                    }
                }
                '^' => {
                    if i == 0 {
                        input[i][j] = 'X';
                        return false;
                    }
                    if input[i-1][j] == '.' || input[i-1][j] == 'X' {
                        input[i][j] = 'X';
                        input[i-1][j] = '^';
                    }
                    if input[i-1][j] == '#' {
                        input[i][j] = '>';
                    }
                }
                _ => {}
            }
        }
    }
    true
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn get_visited_pos_count(map: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'X' {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
        let day = Day6 { input };
        assert_eq!(day.solve_task_one().unwrap(), "41");
    }

    #[test]
    fn test_task_two() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
        let day = Day6 { input };
        assert_eq!(day.solve_task_two().unwrap(), "6");
    }

}