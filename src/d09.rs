use std::collections::VecDeque;

pub struct Row(Vec<u32>);

impl std::str::FromStr for Row {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s.chars().map(|n| n.to_digit(10).expect("Invalid number")).collect();
        Ok(Row(numbers))
    }
}

pub struct LowPoint {
    pub row_index: usize,
    pub item_index: usize,
    pub item: u32,
}

pub fn find_low_points(data: &Vec<Vec<u32>>) -> Vec<LowPoint> {
    let mut low_points = Vec::new();

    for (row_index, row) in data.iter().enumerate() {
        for (item_index, item) in row.iter().enumerate() {
            let item_up = if row_index != 0 { 
                data.get(row_index - 1).and_then(|row| row.get(item_index)).filter(|&&x| x > *item).is_some()
            } else {
                true
            };
            
            let item_down = if row_index != (data.len() - 1) {
                data.get(row_index + 1).and_then(|row| row.get(item_index)).filter(|&&x| x > *item).is_some()
            } else {
                true
            };
            
            let item_left = if item_index != 0 {
                data.get(row_index).and_then(|row| row.get(item_index - 1)).filter(|&&x| x > *item).is_some()
            } else {
                true
            };


            let item_right = if item_index != (row.len() - 1) {
                data.get(row_index).and_then(|row| row.get(item_index + 1)).filter(|&&x| x > *item).is_some()
            } else {
                true
            };
            
            if item_up && item_down && item_left && item_right { low_points.push(LowPoint {  row_index, item_index, item: *item }) }
        }
    }
    low_points
}

pub fn multiply_three_biggest_basins(low_points: Vec<(usize, usize)>, data: Vec<Vec<u32>>) -> usize {
    let mut basins_sizes: Vec<usize> = low_points.into_iter().map(|p| get_basin_size(p, &data)).collect();
    basins_sizes.sort_by(|&a, &b| a.cmp(&b));
    basins_sizes.into_iter().rev().take(3).fold(1, |acc, n| acc * n)
}

pub fn get_basin_size(low_point: (usize, usize), data: &Vec<Vec<u32>>) -> usize {
    let mut to_be_visited: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();

    to_be_visited.push_back(low_point);

    while let Some((x, y)) = to_be_visited.pop_front() {
        visited.push((x, y));
        
        if (x > 0) && data[x-1][y] != 9 && !visited.contains(&(x - 1, y)) { 
            if !to_be_visited.contains(&(x - 1, y)) { to_be_visited.push_back((x - 1, y)) } 
        }

        if (x < data.len() - 1) && data[x+1][y] != 9 && !visited.contains(&(x + 1, y)) {
            if !to_be_visited.contains(&(x + 1, y)) { to_be_visited.push_back((x + 1, y)) }
        }

        if (y > 0) && data[x][y-1] != 9 && !visited.contains(&(x, y - 1)) {
            if !to_be_visited.contains(&(x, y - 1)) { to_be_visited.push_back((x, y - 1)) }
        }

        if (y < data[x].len() - 1) && data[x][y+1] != 9 && !visited.contains(&(x, y + 1)) {
            if !to_be_visited.contains(&(x, y + 1)) { to_be_visited.push_back((x, y + 1)) }
        }
    }
    visited.len()
}


#[cfg(test)]
mod tests {
    
    use crate::data;
    use super::*;

    #[test]
    fn day_9_part_1_solution() {
        let data: Vec<Row> = data::read_input_data("./data/d09/data.txt").expect("Invalid input data");
        let data: Vec<Vec<u32>> = data.into_iter().map(|r| r.0).collect();
        let low_points = find_low_points(&data);
        let output = low_points.into_iter().fold(0, |acc, p| acc + (p.item + 1));
        assert_eq!(output, 558);
    }

    #[test]
    fn day_9_part_2_solution() {
        let data: Vec<Row> = data::read_input_data("./data/d09/data.txt").expect("Invalid input data");
        let data: Vec<Vec<u32>> = data.into_iter().map(|r| r.0).collect();
        let low_points = find_low_points(&data).into_iter().map(|p| (p.row_index, p.item_index)).collect();
        let three_biggest_basins = multiply_three_biggest_basins(low_points, data);
        assert_eq!(three_biggest_basins, 882942);
    }
}