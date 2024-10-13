use crate::utils::point::{Dir, Point};
use std::collections::HashMap;

pub fn part_one(input: &String) -> i32 {
    let mut hist: HashMap<Point, i32> = HashMap::new();
    let mut curr: Point = Point(0, 0);
    hist.insert(Point(0, 0), 1);
    for i in input.trim().chars() {
        curr = match i {
            '^' => curr.step(Dir::North, 1),
            'v' => curr.step(Dir::South, 1),
            '>' => curr.step(Dir::East, 1),
            '<' => curr.step(Dir::West, 1),
            _ => panic!("{:?}", i),
        };
        let count = hist.entry(Point(curr.0, curr.1)).or_insert(0);
        *count += 1
    }
    hist.keys().len() as i32
}
pub fn part_two(_input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_one() {
        let example = String::from(">");
        assert_eq!(part_one(&example), 2);
    }

    #[test]
    fn test_one_1() {
        let example = String::from("^>v<");
        assert_eq!(part_one(&example), 4);
    }

    #[test]
    fn test_one_2() {
        let example = String::from("^v^v^v^v^v");
        assert_eq!(part_one(&example), 2);
    }

    #[test]
    fn test_two() {
        let example = String::from(">");
        assert_eq!(0, part_two(&example));
    }
}
