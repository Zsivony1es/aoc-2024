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
        let mut map = get_map(&self.input);
        let mut time_loop_positions: Vec<(usize, usize)> = Vec::new();

        loop {
            let mut newmap = map.clone();
            let obstacle_pos = place_obstacle_infront(&mut newmap);
            if !time_loop_positions.contains(&obstacle_pos) { 
                let mut count_moves = 0;
                while move_guard(&mut newmap) {
                    if count_moves > 10_000 {
                        break;
                    }
                    count_moves += 1;
                };
                if count_moves > 10_000 && obstacle_pos != (999, 999) {
                    time_loop_positions.push(obstacle_pos);
                    println!("Time loop position: ({}, {})", obstacle_pos.0, obstacle_pos.1);
                }
            }

            if !move_guard(&mut map) {break;};
        };

        Ok(time_loop_positions.len().to_string())
    }
}

fn place_obstacle_infront(map: &mut Vec<Vec<char>>) -> (usize, usize) {
    let (i, j, dir) = get_guard_pos(&map);

    let row_count: usize = map.len();
    let col_count: usize = map[0].len();

    match dir {
        '>' => {
            if j + 1 >= col_count {
                return (999, 999);
            }
            map[i][j+1] = '#';
            (i, j+1)
        }
        '<' => {
            if j == 0 {
                return (999, 999);
            }
            map[i][j-1] = '#';
            (i, j-1)
        }
        'v' => {
            if i + 1 >= row_count {
                return (999, 999);
            }
            map[i+1][j] = '#';
            (i+1, j)
        }
        '^' => {
            if i == 0 {
                return (999, 999);
            }
            map[i-1][j] = '#';
            (i-1, j)
        }
        _ => panic!("Invalid direction!")
    }
}

fn move_guard(input: &mut Vec<Vec<char>>) -> bool {
    let row_count = input.len();
    let col_count = input[0].len();

    let (i, j, dir) = get_guard_pos(input);
    match dir {
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
    true
}

fn get_guard_pos(input: &Vec<Vec<char>>) -> (usize, usize, char) {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '^' {
                return (i, j, '^');
            } else if input[i][j] == 'v' {
                return (i, j, 'v');
            } else if input[i][j] == '>' {
                return (i, j, '>');
            } else if input[i][j] == '<' {
                return (i, j, '<');
            }
        }
    }
    (0, 0, ' ')
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