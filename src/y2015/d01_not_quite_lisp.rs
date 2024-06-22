pub fn part_one(input: &String) -> i32 {
    let mut count = 0;
    let input_as_chars: Vec<char> = input.trim().chars().collect();
    for c in input_as_chars {
        if c == '(' { count += 1 }
        if c == ')' { count -= 1 }
    }
    count
}

pub fn part_two(input: &String) -> i32 {
    let mut position = 0;
    let mut count = 0;
    let input_as_chars: Vec<char> = input.trim().chars().collect();
    for c in input_as_chars {
        if c == '(' { count += 1 }
        if c == ')' { count -= 1 }
        position += 1;
        if count < 0 {
            break;
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_one() {
        let example = String::from("(())");
        assert_eq!(part_one(&example), 0);
    }

    #[test]
    fn test_two() {
        let example = String::from(")())())");
        assert_eq!(part_one(&example), -3);
    }

    #[test]
    fn test_three() {
        let example = String::from(")())())");
        assert_eq!(part_two(&example), 1)
    }
}
