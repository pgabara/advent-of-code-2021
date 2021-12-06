pub struct LanternfishList(Vec<usize>);

impl std::str::FromStr for LanternfishList {

    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        for item in data.split(',') {
            let item: usize = item.parse().map_err(|_| String::from("Invalid item"))?;
            items.push(item);
        }
        Ok(LanternfishList(items))
    }
}

pub fn simulate_lanternfish_grow(lanternfish_data: &Vec<usize>, number_of_cycles: usize) -> usize {
    let mut data = [0; 9];

    for &n in lanternfish_data {
        data[n] += 1;
    }

    (0..number_of_cycles).into_iter().for_each(|_| {
        data.rotate_left(1);
        data[6] += data[8];
    });

    data.into_iter().fold(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn day_6_part_1_solution() {
        let lanternfish_data: LanternfishList = data::read_one_line_input_data("./data/d06/data.txt").expect("Invalid input data");
        let simulation_results = simulate_lanternfish_grow(&lanternfish_data.0, 80);
        assert_eq!(simulation_results, 359344);
    }

    #[test]
    fn day_6_part_2_solution() {
        let lanternfish_data: LanternfishList = data::read_one_line_input_data("./data/d06/data.txt").expect("Invalid input data");
        let simulation_results = simulate_lanternfish_grow(&lanternfish_data.0, 256);
        assert_eq!(simulation_results, 1629570219571);
    }
}