use std::collections::HashMap;

use super::Solve;

use itertools::Itertools;

pub struct Day8 {
    pub input: String
}

impl Solve for Day8 {
    fn solve_task_one(&self) -> Result<String, &'static str> {
        let antennas = get_antennas_with_pos(&self.input);
        let (rows, cols) = get_field_size(&self.input);

        let antinodes = get_antinodes(&antennas);
        let filtered_anodes: Vec<&(i32, i32)> = antinodes
            .iter()
            .filter(|node| {
                node.0 >= 0 && node.0 < rows && node.1 >= 0 && node.1 < cols
            })
            .collect();

        Ok(filtered_anodes.len().to_string())
    }
    
    fn solve_task_two(&self) -> Result<String, &'static str> {
        let antennas = get_antennas_with_pos(&self.input);
        let (rows, cols) = get_field_size(&self.input);

        let antinodes = get_antinodes_pt2(&antennas, (rows, cols));
        let filtered_anodes: Vec<&(i32, i32)> = antinodes
            .iter()
            .filter(|node| {
                node.0 >= 0 && node.0 < rows && node.1 >= 0 && node.1 < cols
            })
            .collect();

        Ok(filtered_anodes.len().to_string())
    }
}

fn get_antennas_with_pos(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (x, row) in input.lines().enumerate() {
        for (y, c) in row.chars().enumerate() {
            if c != '.' {
                if antennas.contains_key(&c){
                    antennas.get_mut(&c).unwrap().push((x as i32, y as i32));
                } else {
                    antennas.insert(c, vec![(x as i32, y as i32)]);
                }
            }
        }
    }
    antennas
}

fn get_antinodes(antennas: &HashMap<char, Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, v) in antennas.iter() {
        v.iter()
        .combinations(2)
        .for_each(|nodepair| {
            let x_diff = (nodepair[0].1 - nodepair[1].1).abs();
            let y_diff = (nodepair[0].0 - nodepair[1].0).abs();

            if nodepair[0].1 < nodepair[1].1 { // [0] is left of [1]
                if nodepair[0].0 < nodepair[1].0 { // [0] is above [1]
                    antinodes.push((nodepair[0].1 - x_diff, nodepair[0].0 - y_diff));
                    antinodes.push((nodepair[1].1 + x_diff, nodepair[1].0 + y_diff));
                } else { // [0] is below [1]
                    antinodes.push((nodepair[0].1 - x_diff, nodepair[0].0 + y_diff));
                    antinodes.push((nodepair[1].1 + x_diff, nodepair[1].0 - y_diff));
                }
            } else { // [0] is right of [1]
                if nodepair[0].0 < nodepair[1].0 {
                    antinodes.push((nodepair[0].1 + x_diff, nodepair[0].0 - y_diff));
                    antinodes.push((nodepair[1].1 - x_diff, nodepair[1].0 + y_diff));
                } else {
                    antinodes.push((nodepair[0].1 + x_diff, nodepair[0].0 + y_diff));
                    antinodes.push((nodepair[1].1 - x_diff, nodepair[1].0 - y_diff));
                }
            }
        });
    }
    antinodes = antinodes.iter().unique().cloned().collect();
    antinodes
}

fn get_antinodes_pt2(antennas: &HashMap<char, Vec<(i32, i32)>>, mapsize: (i32, i32)) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, v) in antennas.iter() {
        v.iter()
        .combinations(2)
        .for_each(|nodepair| {
            let x_diff = (nodepair[0].1 - nodepair[1].1).abs();
            let y_diff = (nodepair[0].0 - nodepair[1].0).abs();

            if nodepair[0].1 < nodepair[1].1 { // [0] is left of [1]
                if nodepair[0].0 < nodepair[1].0 { 
                    let mut counter = 0;// [0] is above [1]
                    loop {
                        let anode1 = (nodepair[0].1 - counter * x_diff, nodepair[0].0 - counter * y_diff);
                        let anode2 = (nodepair[1].1 + counter * x_diff, nodepair[1].0 + counter * y_diff);
                        if !on_map(anode1, mapsize) && !on_map(anode2, mapsize) {break;}
                        antinodes.push(anode1);
                        antinodes.push(anode2);
                        counter += 1;
                    }
                } else { // [0] is below [1]
                    let mut counter = 0;// [0] is above [1]
                    loop {
                        let anode1 = (nodepair[0].1 - counter * x_diff, nodepair[0].0 + counter * y_diff);
                        let anode2 = (nodepair[1].1 + counter * x_diff, nodepair[1].0 - counter * y_diff);
                        if !on_map(anode1, mapsize) && !on_map(anode2, mapsize) {break;}
                        antinodes.push(anode1);
                        antinodes.push(anode2);
                        counter += 1;
                    }
                }
            } else { // [0] is right of [1]
                if nodepair[0].0 < nodepair[1].0 {
                    let mut counter = 0;// [0] is above [1]
                    loop {
                        let anode1 = (nodepair[0].1 + counter * x_diff, nodepair[0].0 - counter * y_diff);
                        let anode2 = (nodepair[1].1 - counter * x_diff, nodepair[1].0 + counter * y_diff);
                        if !on_map(anode1, mapsize) && !on_map(anode2, mapsize) {break;}
                        antinodes.push(anode1);
                        antinodes.push(anode2);
                        counter += 1;
                    }
                } else {
                    let mut counter = 0;// [0] is above [1]
                    loop {
                        let anode1 = (nodepair[0].1 + counter * x_diff, nodepair[0].0 + counter * y_diff);
                        let anode2 = (nodepair[1].1 - counter * x_diff, nodepair[1].0 - counter * y_diff);
                        if !on_map(anode1, mapsize) && !on_map(anode2, mapsize) {break;}
                        antinodes.push(anode1);
                        antinodes.push(anode2);
                        counter += 1;
                    }
                }
            }
        });
    }
    antinodes = antinodes.iter().unique().cloned().collect();
    antinodes
}

fn on_map(pos: (i32, i32), mapsize: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < mapsize.0 && pos.1 >= 0 && pos.1 < mapsize.1
}

fn get_field_size(input: &str) -> (i32, i32) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    (rows as i32, cols as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".to_string();
        let day = Day8 { input };
        assert_eq!(day.solve_task_one().unwrap(), "14");
    }

    #[test]
    fn test_task_two() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............".to_string();
        let day = Day8 { input };
        assert_eq!(day.solve_task_two().unwrap(), "34");
    }

}