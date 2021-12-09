use std::collections::HashSet;

type Pattern = Vec<char>;

#[derive(Debug)]
pub struct Entry {
    test: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl std::str::FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.splitn(2, " | ");
        let test: Vec<Vec<char>> = data.next().unwrap().split_ascii_whitespace().map(|s| s.chars().collect()).collect();
        let output: Vec<Vec<char>> = data.next().unwrap().split_ascii_whitespace().map(|s| s.chars().collect()).collect();
        Ok(Entry { test, output })
    }
}

pub fn calculate_known_digits(data: Vec<Entry>) -> usize {
    data.into_iter()
        .map(|e| e.output)
        .map(|patterns| {
            let x: Vec<_> = patterns.into_iter()
                .filter(|pattern| {
                    pattern.len() == 2 || pattern.len() == 3 || pattern.len() == 4 || pattern.len() == 7
                })
                .collect();
            x.len()
        })
        .sum()
}

pub fn parse_entries_and_sum(data: Vec<Entry>) -> usize {

    let mut result = 0;

    for entry in data {
        let mut mappings: Vec<Vec<char>> = vec![Vec::new(); 10];
        let mut test_data = entry.test;
        test_data.sort_unstable_by_key(|a| a.len());
        for test_pattern in test_data {
            if test_pattern.len() == 2 { mappings[1] = test_pattern }
            else if test_pattern.len() == 3 { mappings[7] = test_pattern }
            else if test_pattern.len() == 4 { mappings[4] = test_pattern }
            else if test_pattern.len() == 5 {
                if is_three(&test_pattern, &mappings) { mappings[3] = test_pattern }
                else if is_five(&test_pattern, &mappings) { mappings[5] = test_pattern }
                else { mappings[2] = test_pattern }
            }
            else if test_pattern.len() == 6 {
                if is_nine(&test_pattern, &mappings) { mappings[9] = test_pattern }
                else if is_six(&test_pattern, &mappings) { mappings[6] = test_pattern }
                else { mappings[0] = test_pattern }

            }
            else if test_pattern.len() == 7 { mappings[8] = test_pattern }
            else { panic!("You should not be here !") }
        }

        let output = entry.output.into_iter().fold(String::new(), |acc, s| {
            format!("{}{}", acc, parse_pattern(s, &mappings).expect("Invalid pattern"))
        });

        let output: usize = output.parse().expect("Invalid number");
        result += output;
    }

    result
}

fn parse_pattern(pattern: Vec<char>, mappings: &Vec<Vec<char>>) -> Option<String> {
    for (index, mapping) in mappings.into_iter().enumerate() {
        if are_patterns_equal(&pattern, mapping) { return Some(index.to_string()) }
    }
    None
}

fn are_patterns_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    let a: HashSet<&char> = HashSet::from_iter(a.iter());
    let b: HashSet<&char> = HashSet::from_iter(b.iter());
    a == b
}

fn is_three(input: &Vec<char>, mappings: &Vec<Vec<char>>) -> bool {
    let x: Vec<_> = mappings[1].iter().filter(|&x| !input.contains(x)).collect();
    x.is_empty()
}

fn is_five(input: &Vec<char>, mappings: &Vec<Vec<char>>) -> bool {
    let x: Vec<_> = mappings[4].iter().filter(|&x| !input.contains(x)).collect();
    x.len() == 1
}

fn is_nine(input: &Vec<char>, mappings: &Vec<Vec<char>>) -> bool {
    let five: Vec<_> =  mappings[5].iter().filter(|&x| !input.contains(x)).collect();
    let one: Vec<_> = mappings[1].iter().filter(|&x| !input.contains(x)).collect();
    five.is_empty() && one.is_empty()
}

fn is_six(input: &Vec<char>, mappings: &Vec<Vec<char>>) -> bool {
    let x: Vec<_> =  mappings[5].iter().filter(|&x| !input.contains(x)).collect();
    x.is_empty()
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn day_8_part_1_solution() {
        let data: Vec<Entry> = data::read_input_data("./data/d08/data.txt").expect("Invalid input data");
        let known_digits = calculate_known_digits(data);
        assert_eq!(known_digits, 261);
    }

    #[test]
    fn day_8_part_2_solution() {
        let data: Vec<Entry> = data::read_input_data("./data/d08/data.txt").expect("Invalid input data");
        let output = parse_entries_and_sum(data);
        assert_eq!(output, 987553);
    }
}