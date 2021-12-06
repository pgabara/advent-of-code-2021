use std::io::{Error, BufReader, BufRead};
use std::str::FromStr;

pub fn read_input_data<T: FromStr>(path: &str) -> Result<Vec<T>, Error> {
    let mut data = Vec::new();
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let parsed_data: T = line?.parse().map_err(|_| Error::from_raw_os_error(22))?;
        data.push(parsed_data);

    }
    Ok(data)
}

pub fn read_one_line_input_data<T: FromStr>(path: &str) -> Result<T, Error> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().ok_or(Error::from_raw_os_error(22))?;
    let line = line.map_err(|_| Error::from_raw_os_error(22))?;
    line.parse().map_err(|_| Error::from_raw_os_error(22))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_input_data_to_vec() {
        let data: Vec<i32> = read_input_data("./data/d01/data.txt").expect("Invalid input data");
        assert_eq!(data.len(), 2000);
        let first_five_numbers: Vec<i32> = data.into_iter().take(5).collect();
        assert_eq!(first_five_numbers, vec![141, 152, 164, 163, 164]);
    }

    #[test]
    fn read_one_line_input_data_to_string() {
        let data: String = read_one_line_input_data("./data/d06/data.txt").expect("Invalid input data");
        let data: Vec<&str> = data.split(',').take(5).collect();
        let expected_data = vec!["4", "3", "4", "5", "2"];
        assert_eq!(data, expected_data);
    }   
}