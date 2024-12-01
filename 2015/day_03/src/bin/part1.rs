use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

#[test]
fn it_works() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    assert_eq!(result, 2);
}

#[derive(Hash, Eq, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

fn parse_input(input: &str) -> usize
{
    let mut curr_x: isize = 0;
    let mut curr_y: isize = 0;
    let mut visited_houses = HashSet::new();
    visited_houses.insert(Vec2 { x: curr_x, y: curr_y });

    for c in input.chars() { 
        if c == '^' {
            curr_y += 1;
        }
        if c == '>' {
            curr_x += 1;
        }
        if c == 'v' {
            curr_y -= 1;
        }
        if c == '<' {
            curr_x -= 1;
        }

        visited_houses.insert(Vec2 { x: curr_x, y: curr_y });
    }

    return visited_houses.len();

}