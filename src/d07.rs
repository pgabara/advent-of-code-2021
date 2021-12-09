pub struct Crabs(Vec<i32>);

impl std::str::FromStr for Crabs {

    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut output = Vec::new();
        for n in data.split(',') {
            let n: i32 = n.parse().map_err(|_| String::from("Invalid number"))?;
            output.push(n);
        }
        Ok(Crabs(output))
    }
}

pub fn calculate_fuel_cost(crabs: &Vec<i32>, distance_to_cost: fn(i32) -> i32) -> i32 {
    let min_position = *crabs.iter().min().unwrap_or(&i32::MIN);
    let max_position = *crabs.iter().max().unwrap_or(&i32::MAX);
    let mut target_cost = i32::MAX;

    for next_position in (min_position..max_position).into_iter() {
        let cost = crabs.iter().fold(0, |acc, &next| {
            let cost = distance_to_cost((next - next_position).abs());
            cost + acc
        });
        if cost < target_cost { target_cost = cost }
    }

    target_cost
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn day_7_part_1_solution() {
        let data: Crabs = data::read_one_line_input_data("./data/d07/data.txt").expect("Invalid input data");
        let fuel_cost = calculate_fuel_cost(&data.0, |distance| distance);
        assert_eq!(fuel_cost, 336120);
    }

    #[test]
    fn day_7_part_2_solution() {
        let data: Crabs = data::read_one_line_input_data("./data/d07/data.txt").expect("Invalid input data");
        // let fuel_cost = calculate_fuel_cost(&data.0, |n| (1..=n).sum());
        let fuel_cost = calculate_fuel_cost(&data.0, |distance| (distance * (distance + 1)) / 2);
        assert_eq!(fuel_cost, 96864235);
    }
}