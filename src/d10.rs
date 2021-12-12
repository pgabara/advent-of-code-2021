use std::collections::VecDeque;

pub struct Row(Vec<char>);

impl std::str::FromStr for Row {
    
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect();
        Ok(Row(chars))
    }
}

fn char_to_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn find_corrupted_char(chars: &Vec<char>) -> Option<char> {
    let mut expected_closing_chars: VecDeque<char> = VecDeque::new();

    for &c in chars {
        match c {
            '(' => expected_closing_chars.push_back(')'),
            '[' => expected_closing_chars.push_back(']'),
            '{' => expected_closing_chars.push_back('}'),
            '<' => expected_closing_chars.push_back('>'),
            ')' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != ')' { return Some(')') };
                }
            },
            ']' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != ']' { return Some(']') };
                }
            },
            '}' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != '}' { return Some('}') };
                }
            }
            '>' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != '>' { return Some('>') };
                }
            },
            _ => ()
        }
    }

    None
}

fn analyze_chars(chars: &Vec<char>) -> Result<VecDeque<char>, char> {
    let mut expected_closing_chars: VecDeque<char> = VecDeque::new();

    for &c in chars {
        match c {
            '(' => expected_closing_chars.push_back(')'),
            '[' => expected_closing_chars.push_back(']'),
            '{' => expected_closing_chars.push_back('}'),
            '<' => expected_closing_chars.push_back('>'),
            ')' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != ')' { return Err(')') };
                }
            },
            ']' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != ']' { return Err(']') };
                }
            },
            '}' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != '}' { return Err('}') };
                }
            }
            '>' => {
                if let Some(e) = expected_closing_chars.pop_back() {
                    if e != '>' { return Err('>') };
                }
            },
            _ => ()
        }
    }

    Ok(expected_closing_chars)
}

pub fn find_middle_completions_score(data: Vec<Vec<char>>) -> usize {
    let mut scores = Vec::new();
    for row in data {
        match analyze_chars(&row) {
            Ok(xs) => {
                let score = calculate_score(xs);
                scores.push(score);
            },
            Err(_) => (),
        }
    }
    let index = scores.len() / 2;
    scores.sort_by(|a, b| a.cmp(b));
    scores[index]
}

fn calculate_score(mut input: VecDeque<char>) -> usize {
    let mut total_score = 0;

    while let Some(x) = input.pop_back()  {
        total_score *= 5;
        match x {
            ')' => total_score += 1,
            ']' => total_score += 2,
            '}' => total_score += 3,
            '>' => total_score += 4,
            _   => (),
        }
    }

    total_score
}

pub fn calculate_corrupted_chars(data: Vec<Vec<char>>) -> usize {
    data.into_iter()
        .map(|chars| find_corrupted_char(&chars))
        .map(|output| {
            match output {
                None => 0,
                Some(c) => char_to_score(c),
            }
        })
        .fold(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    
    use crate::data;
    use super::*;

    #[test]
    fn day_10_part_1_solution() {
        let data: Vec<Row> = data::read_input_data("./data/d10/data.txt").expect("Invalid input data");
        let data: Vec<Vec<char>> = data.into_iter().map(|r| r.0).collect();
        let score = calculate_corrupted_chars(data);
        assert_eq!(score, 266301);
    }

    #[test]
    fn day_10_part_2_solution() {
        let data: Vec<Row> = data::read_input_data("./data/d10/data.txt").expect("Invalid input data");
        let data: Vec<Vec<char>> = data.into_iter().map(|r| r.0).collect();
        let score = find_middle_completions_score(data);
        assert_eq!(score, 3404870164);
    }
}