use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point { 
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

#[derive(Debug)]
pub struct VentsLine {
    pub from: Point,
    pub to: Point,
}

impl VentsLine {

    pub fn is_vertical(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn is_vertical_or_horizontal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_vertically_decreased(&self) -> bool {
        if self.from.x > self.to.x { 
            self.from.y < self.to.y
        } else {
            self.from.y > self.to.y
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let min = usize::min(self.from.x, self.to.x);
            let max = usize::max(self.from.x, self.to.x) + 1;
            (min..max).into_iter().map(|n| Point::new(n, self.from.y)).collect()
        } else if self.is_horizontal() {
            let min = usize::min(self.from.y, self.to.y);
            let max = usize::max(self.from.y, self.to.y) + 1;
            (min..max).into_iter().map(|n| Point::new(self.from.x, n)).collect()
        } else {
            let min_x = usize::min(self.from.x, self.to.x);
            let max_x = usize::max(self.from.x, self.to.x) + 1;
            let min_y = usize::min(self.from.y, self.to.y);
            let max_y = usize::max(self.from.y, self.to.y) + 1;
            let range_x = (min_x..max_x).into_iter();
            let range_y = (min_y..max_y).into_iter();
            if self.is_vertically_decreased() { range_x.zip(range_y.rev()).map(|(x, y)| Point::new(x, y)).collect() }
            else { range_x.zip(range_y).map(|(x, y)| Point::new(x, y)).collect() }
            
        }
    }
}

type VentsState = HashMap<Point, usize>;

fn feed_vents_state(vents_lines: &Vec<VentsLine>) -> VentsState {
    let mut state = VentsState::new();

    for vent_line in vents_lines.into_iter() {
        let points = vent_line.get_points();
        points.into_iter().for_each(|p| { *state.entry(p).or_insert(0) += 1; });
    }

    state
}

pub fn find_number_of_points_that_overlap(vents_lines: &Vec<VentsLine>) -> usize {
    let state = feed_vents_state(vents_lines);
    let overlaps: Vec<_> = state.into_iter().filter(|&(_, n)| { n > 1 }).collect();
    overlaps.len()
}

impl std::str::FromStr for VentsLine {

    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut data = data.splitn(2, " -> ");
        let from = data.next().ok_or(String::from("No 'from' point"))?;
        let to = data.next().ok_or(String::from("No 'to' point"))?;
        let mut from = from.splitn(2, ',');
        let mut to = to.splitn(2, ',');
        let from_x: usize = from.next().and_then(|x| x.parse().ok()).ok_or("No 'from' x")?;
        let from_y: usize = from.next().and_then(|y| y.parse().ok()).ok_or("No 'from' y")?;
        let to_x: usize = to.next().and_then(|x| x.parse().ok()).ok_or("No 'to' x")?;
        let to_y: usize = to.next().and_then(|y| y.parse().ok()).ok_or("No 'to' y")?;
        Ok(VentsLine { from: Point::new(from_x, from_y), to: Point::new(to_x, to_y) })
    }
}

#[cfg(test)]
mod tests {

    use crate::data;
    use super::*;

    #[test]
    fn is_vertical() {
        let v = VentsLine { from: Point::new(0, 10), to: Point::new(10, 10) };
        assert!(v.is_vertical_or_horizontal())
    }

    #[test]
    fn is_horizontal() {
        let v = VentsLine { from: Point::new(0, 10), to: Point::new(0, 20) };
        assert!(v.is_vertical_or_horizontal())
    }

    #[test]
    fn neither_vertical_or_horizontal() {
        let v = VentsLine { from: Point::new(0, 10), to: Point::new(10, 20) };
        assert_eq!(v.is_vertical_or_horizontal(), false)
    }

    #[test]
    fn get_vertical_point() {
        let v = VentsLine { from: Point::new(0, 10), to: Point::new(2, 10) };
        let expected_points = vec![Point::new(0, 10), Point::new(1, 10), Point::new(2, 10)];
        assert_eq!(v.get_points(), expected_points);

        let v = VentsLine { from: Point::new(2, 10), to: Point::new(0, 10) };
        assert_eq!(v.get_points(), expected_points);
    }

    #[test]
    fn get_horizontal_points() {
        let v = VentsLine { from: Point::new(0, 0), to: Point::new(0, 2) };
        let expected_points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)];
        assert_eq!(v.get_points(), expected_points);

        let v = VentsLine { from: Point::new(0, 2), to: Point::new(0, 0) };
        assert_eq!(v.get_points(), expected_points);
    }

    #[test]
    fn get_diagonal_points() {
        let v = VentsLine { from: Point::new(1, 1), to: Point::new(3, 3) };
        let expected_points = vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)];
        assert_eq!(v.get_points(), expected_points);

        let v = VentsLine { from: Point::new(3, 3), to: Point::new(1, 1) };
        assert_eq!(v.get_points(), expected_points);

        let v = VentsLine { from: Point::new(9, 7), to: Point::new(7, 9) };
        let expected_points = vec![Point::new(7, 9), Point::new(8, 8), Point::new(9, 7)];
        assert_eq!(v.get_points(), expected_points);
    }

    #[test]
    fn day_5_part_1_solution() {
        let data: Vec<VentsLine> = data::read_input_data("./data/d05/data.txt").expect("Invalid input data");
        let data: Vec<VentsLine> = data.into_iter().filter(|v| v.is_vertical_or_horizontal()).collect();
        let number_of_overlaps = find_number_of_points_that_overlap(&data);
        assert_eq!(number_of_overlaps, 5632);
    }

    #[test]
    fn day_5_part_2_solution() {
        let data: Vec<VentsLine> = data::read_input_data("./data/d05/data.txt").expect("Invalid input data");
        let number_of_overlaps = find_number_of_points_that_overlap(&data);
        assert_eq!(number_of_overlaps, 22213);
    }
}