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
        let mut time_loop_positions: Vec<[usize; 2]> = Vec::new();

        let row_count: usize = map.len();
        let col_count: usize = map[0].len();

        while move_guard(&mut map) {
            let (i, j, dir) = get_guard_pos(&map);
            match dir {
                '>' => {
                    if i + 1 < row_count && j + 1 < col_count &&
                      map[i+1][j] == 'X' && 
                      !time_loop_positions.contains(&[i,j+1]) {

                        let mut loop_i = i;
                        while loop_i + 1 < row_count && map[loop_i][j] == 'X' {
                            loop_i += 1;
                        }
                        if loop_i + 1 >= row_count {
                            continue;
                        } else if (map[loop_i][j] == '#') {
                            time_loop_positions.push([i,j+1]);
                        }

                    }
                },
                'v' => {
                    if j >= 1 && i + 1 < row_count &&
                      map[i][j-1] == 'X' && 
                      !time_loop_positions.contains(&[i+1,j]) {

                        let mut loop_j: i32 = j as i32;
                        while loop_j >= 0 && map[i][loop_j as usize] == 'X' {
                            loop_j -= 1;
                        }
                        if loop_j < 0 {
                            continue;
                        } else if map[i][(loop_j + 1) as usize] == '#' {
                            time_loop_positions.push([i+1,j]);
                        }

                    }
                },
                '<' => {
                    if i >= 1 && j >= 1  &&
                      map[i-1][j] == 'X' && 
                      !time_loop_positions.contains(&[i,j-1]) {

                        let mut loop_i: i32 = i as i32;
                        while loop_i - 1 >= 0 && map[loop_i as usize][j] == 'X' {
                            loop_i -= 1;
                        }
                        if loop_i < 0 {
                            continue;
                        } else if map[(loop_i + 1) as usize][j] == '#' {
                            time_loop_positions.push([i,j+1]);
                        } 
                    }
                },
                '^' => {
                    if i >= 1 && j + 1 < col_count  &&
                      map[i][j+1] == 'X' && 
                      !time_loop_positions.contains(&[i-1,j]) {
                        
                        let mut loop_j = j;
                        while loop_j + 1 < col_count && map[i][loop_j] == 'X' {
                            loop_j += 1;
                        }
                        if loop_j + 1 >= col_count {
                            continue;
                        } else if map[i][loop_j] == '#' {
                            time_loop_positions.push([i+1,j]);
                        }

                    }
                },
                _ => ()
            }
        }

        println!("{:?}", time_loop_positions);

        Ok(time_loop_positions.len().to_string())
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