use core::str;

use super::Solve;

pub struct Day4 {
    pub input: String
}

impl Solve for Day4 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        Ok(find_all_task1(&self.input).to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        Ok(find_all_task2(&self.input).to_string())
    }
}

fn find_all_task1(input: &String) -> i32 {
    let mut xmas_count = 0;

    let letters = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

    let col_count = letters[0].len();
    let row_count = letters.len();

    // Horizontal matches
    for i in 0..col_count-3 {
        for j in 0..row_count {
            let mut str = String::new();
            str.push(letters[j][i]);
            str.push(letters[j][i+1]);
            str.push(letters[j][i+2]);
            str.push(letters[j][i+3]);
            if str == "XMAS" || str == "SAMX" {
                xmas_count += 1;
            }
        }
    }

    // Vertical matches
    for i in 0..col_count {
        for j in 0..row_count-3 {
            let mut str = String::new();
            str.push(letters[j][i]);
            str.push(letters[j+1][i]);
            str.push(letters[j+2][i]);
            str.push(letters[j+3][i]);
            if str == "XMAS" || str == "SAMX" {
                xmas_count += 1;
            }
        }
    }

    // Diagonal matches
    for i in 0..col_count-3 {
        for j in 0..row_count-3 {
            let mut topleft = String::new();
            let mut topright = String::new();

            topleft.push(letters[j][i]);
            topleft.push(letters[j+1][i+1]);
            topleft.push(letters[j+2][i+2]);
            topleft.push(letters[j+3][i+3]);

            topright.push(letters[j][i+3]);
            topright.push(letters[j+1][i+2]);
            topright.push(letters[j+2][i+1]);
            topright.push(letters[j+3][i]);
            
            if topleft == "XMAS" || topleft == "SAMX" {
                xmas_count += 1;
            }
            if topright == "XMAS" || topright == "SAMX" {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

fn find_all_task2(input: &String) -> i32 {
    let mut mas_count = 0;

    let letters = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

    let col_count = letters[0].len();
    let row_count = letters.len();

    for i in 0..col_count-2 {
        for j in 0..row_count-2 {
            let mut topleft = String::new();
            let mut topright = String::new();

            topleft.push(letters[j][i]);
            topleft.push(letters[j+1][i+1]);
            topleft.push(letters[j+2][i+2]);

            topright.push(letters[j][i+2]);
            topright.push(letters[j+1][i+1]);
            topright.push(letters[j+2][i]);
            
            if (topleft == "MAS" || topleft == "SAM") && (topright == "MAS" || topright == "SAM") {
                mas_count += 1;
            }
        }
    }
    mas_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_all_words_in_example_task1(){
        let letters = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();

        assert_eq!(find_all_task1(&letters), 18);
    }

    #[test]
    fn finds_all_words_in_example_task2(){
        let letters = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();

        assert_eq!(find_all_task2(&letters), 9);
    }
    
}