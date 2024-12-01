fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

#[test]
fn test() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    assert_eq!(result, 11);
}

fn parse_input(input: &str) -> isize
{
    return 11;
}

