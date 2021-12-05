use std::io::BufRead;

pub struct Board(Vec<Vec<Option<usize>>>);

impl Board {

    pub fn new(data: &Vec<Vec<usize>>) -> Self {
        let data = data.iter().map(|row| { 
            row.iter().map(|&n| Some(n)).collect() }
        ).collect();
        Self(data)
    }
    pub fn sum_unmarked(&self) -> usize {
        self.0.iter()
            .flat_map(|xs| xs.iter())
            .fold(0, |acc, x| { if let Some(x) = x { acc + x } else { acc }  })
    }

    pub fn mark_number(&mut self, number: usize) -> Result<bool, String> {
        let mut found_number = None;

        for (row_index, row) in self.0.iter_mut().enumerate() {
            for (item_index, item) in row.iter_mut().enumerate() {
                if *item == Some(number) { 
                    *item = None;
                    found_number = Some((row_index, item_index));
                }
            }
        }
        
        match found_number {
            Some((row, column)) => {
                let is_winning_row = self.is_winning_row(row)?;
                let is_winning_column = self.is_winning_column(column)?;
                Ok(is_winning_row || is_winning_column)
            }
            None => Ok(false)
        }
    }

    pub fn is_winning_row(&self, index: usize) -> Result<bool, String> {
        let row = self.0.get(index).ok_or(String::from("Invalid row index"))?;
        let some_exists = row.iter().find(|item| item.is_some());
        Ok(some_exists.is_none())
    }

    pub fn is_winning_column(&self, index: usize) -> Result<bool, String> {
        for row in self.0.iter() {
            let item = row.get(index).ok_or(String::from("Invalid item index"))?;
            if item.is_some() { return Ok(false) }
        }
        Ok(true)
    }
}

pub struct Bingo {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

impl Bingo {

    pub fn run_simulation(&mut self) -> Result<(usize, usize, usize), String> {
        for next_number in &self.numbers {
            for (board_index, board) in self.boards.iter_mut().enumerate() {
                let is_winning_number = board.mark_number(*next_number)?;
                if is_winning_number {
                    let sum_unmarked = board.sum_unmarked();
                    return Ok((*next_number, sum_unmarked, board_index))
                }
            }
        }
        Err(String::from("No winner"))
    }

    pub fn rust_simulation_last_win(&mut self) -> Result<(usize, usize), String> {
        let mut number_of_boards = self.boards.len();
        while number_of_boards > 0 {
            let (next_number, next_sum, board_index) = self.run_simulation()?;
            self.boards.remove(board_index);
            number_of_boards = self.boards.len();
            if number_of_boards == 0 { return Ok((next_number, next_sum)) }
        }
        Err(String::from("No winner"))
    }

    pub fn init(data_path: &str, board_size: usize) -> Result<Self, String> {
        let file = std::fs::File::open(data_path).map_err(|_| String::from("Unable to read data"))?;
        let reader = std::io::BufReader::new(file);
        let mut lines = reader.lines();

        let numbers = lines.next().ok_or(String::from("No bingo numbers"))?;
        let numbers = numbers.map_err(|_| String::from("Unable to read next line"))?;
        let numbers = Bingo::read_numbers(numbers, false)?;

        let mut boards = Vec::new();
        let mut board_rows = Vec::new();

        for line in lines {
            let line = line.map_err(|_| String::from("Unable to read next line"))?;
            if !line.is_empty() {
                let numbers = Bingo::read_numbers(line, true)?;
                board_rows.push(numbers);
                if board_rows.len() == board_size { 
                    boards.push(Board::new(&board_rows));
                    board_rows = Vec::new();
                 }
            }
        }

        Ok(Self { numbers, boards })
    }

    fn read_numbers(data: String, split_white_spaces: bool) -> Result<Vec<usize>, String> {
        let mut numbers = Vec::new();
        let iter: Box<dyn Iterator<Item = &str>> = if split_white_spaces {
            Box::new(data.split_ascii_whitespace())
        } else {
            Box::new(data.split(','))
        };
        for number in iter {
            let number: usize = number.parse().map_err(|_| String::from("Invalid number"))?;
            numbers.push(number);
        }
        Ok(numbers)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day_4_part_1_solution() {
        let mut bingo: Bingo = Bingo::init("./data/d04/data.txt", 5).expect("Invalid input data");
        let (winning_board, unmarked_summed, _) = bingo.run_simulation().expect("Bingo error");
        assert_eq!(winning_board * unmarked_summed, 58374);
    }

    #[test]
    fn day_4_part_2_solution() {
        let mut bingo: Bingo = Bingo::init("./data/d04/data.txt", 5).expect("Invalid input data");
        let (winning_board, unmarked_summed) = bingo.rust_simulation_last_win().expect("Bingo error");
        assert_eq!(winning_board * unmarked_summed, 11377);
    }
}