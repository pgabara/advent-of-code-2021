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
}