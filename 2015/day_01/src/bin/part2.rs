fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

fn parse_input(input: &str) -> isize
{
    let mut floor_change: isize = 0;
    let mut floor_change_index: isize = 0;

    for c in input.chars() { 
        floor_change_index += 1;
        
        if c == '(' {
            floor_change += 1;
        }
        if c == ')' {
            floor_change -= 1;
        }

        if floor_change < 0 {
            return floor_change_index;
        }
    }

    return floor_change;
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("./input.txt");
        let result = parse_input(input);
        assert_eq!(result, 5);
    }
}


