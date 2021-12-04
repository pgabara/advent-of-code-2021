use std::str::FromStr;

struct Bits(Vec<u8>);

impl FromStr for Bits {

    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::new();
        for c in data.trim().chars() {
            let bit = u8::from_str_radix(&c.to_string(), 2).map_err(|_| String::from("Invalid bit"))?;
            bits.push(bit);
        }
        Ok(Bits(bits))
    }
}

fn find_most_common_bits(data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut ones_count = [0; 12];
    for bits in data {
        for (index, &bit) in bits.iter().enumerate() {
            if bit == 1 { ones_count[index] += 1 } else { ones_count[index] -= 1 }
        }
    }
    ones_count.iter().map(|&count| { if count >= 0 { 1 } else { 0 } }).collect()
}

fn bits_to_usize(bits: &Vec<u8>) -> Result<usize, String> {
    let bits_str = bits.iter().fold(String::new(), |acc, bit| { format!("{}{}", acc, bit) });
    usize::from_str_radix(&bits_str, 2).map_err(|_| String::from("Invalid bits"))
}

pub fn calculate_power_consumption(data: &Vec<Vec<u8>>) -> Result<usize, String> {
    let gamma_rate_raw = find_most_common_bits(data);
    let epsilon_rate_raw = gamma_rate_raw.iter().map(|b| b ^ 1).collect();
    let gamma_rate = bits_to_usize(&gamma_rate_raw)?;
    let epsilon_rate = bits_to_usize(&epsilon_rate_raw)?;
    Ok(gamma_rate * epsilon_rate)
}

fn filter_data_for_life_support_rating(data: &Vec<Vec<u8>>, f: fn(u8) -> u8) -> Result<usize, String> {
    let mut index = 0;
    let mut remaining_data = data.clone();

    while remaining_data.len() > 1 {
        let most_common_bits = find_most_common_bits(&remaining_data);
        let bit = f(most_common_bits[index]);
        remaining_data.retain(|bits| bits[index] == bit);
        index += 1;
    }

    let record = remaining_data.first().ok_or(String::from("No remaining data"))?;
    bits_to_usize(record)
}

pub fn calculate_life_support_rating(data: &Vec<Vec<u8>>) -> Result<usize, String> {
    let oxygen_rate = filter_data_for_life_support_rating(data, |b| b)?;
    let c02_scrubber_rate = filter_data_for_life_support_rating(data, |b| b ^ 1)?;
    Ok(oxygen_rate * c02_scrubber_rate)
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;
    
    #[test]
    fn day_3_part_1_solution() {
        let data: Vec<Bits> = data::read_input_data("./data/d03/data.txt").expect("Invalid input data");
        let data: Vec<Vec<u8>> = data.into_iter().map(|bs| bs.0).collect();
        let power_consumption = calculate_power_consumption(&data).expect("Invalid data");
        assert_eq!(power_consumption, 2261546);
    }

    #[test]
    fn day_3_part_2_solution() {
        let data: Vec<Bits> = data::read_input_data("./data/d03/data.txt").expect("Invalid input data");
        let data: Vec<Vec<u8>> = data.into_iter().map(|bs| bs.0).collect();
        let life_time_support_rating = calculate_life_support_rating(&data).expect("Invalid data");
        assert_eq!(life_time_support_rating, 6775520);
    }
}