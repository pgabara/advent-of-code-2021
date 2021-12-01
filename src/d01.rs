pub fn count_number_of_times_depth_measurement_increases(data: &Vec<usize>) -> usize {
    count_depth_measurement_increases(data)
}

pub fn count_number_of_times_depth_measurement_window_increases(data: &Vec<usize>) -> usize {
    let data: Vec<usize> = data.windows(3).map(|x| x.iter().sum()).collect();
    count_depth_measurement_increases(&data)
}

fn count_depth_measurement_increases(data: &Vec<usize>) -> usize {
    data.windows(2).fold(0, |acc, x| {
        if let &[a, b] = x {
            if b > a { acc + 1 } else { acc }
        } else { 
            acc 
        }
    })
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn day_1_part_1_solution() {
        let data: Vec<usize> = data::read_input_data("./data/d01/data.txt").expect("Invalid input data");
        let depth_count = count_number_of_times_depth_measurement_increases(&data);
        assert_eq!(depth_count, 1184);
    }

    #[test]
    fn day_1_part_2_solution() {
        let data: Vec<usize> = data::read_input_data("./data/d01/data.txt").expect("Invalid input data");
        let depth_count = count_number_of_times_depth_measurement_window_increases(&data);
        assert_eq!(depth_count, 1158);
    }
}