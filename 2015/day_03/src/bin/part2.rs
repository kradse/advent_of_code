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

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

fn parse_input(input: &str) -> usize
{
    let mut visited_houses = HashSet::new();
    visited_houses.insert(Vec2 { x: 0, y: 0 });

    let mut santa_pos = Vec2 { x: 0, y: 0 };
    let mut robo_pos = Vec2 { x: 0, y: 0 };

    let mut santa_turn = true;

    for c in input.chars() {
        if c == '^' {
            if santa_turn {
                santa_pos.y += 1;
            } else {
                robo_pos.y += 1;
            }
        }
        if c == '>' {
            if santa_turn {
                santa_pos.x += 1;
            } else {
                robo_pos.x += 1;
            }
        }
        if c == 'v' {
            if santa_turn {
                santa_pos.y -= 1;
            } else {
                robo_pos.y -= 1;
            }
        }
        if c == '<' {
            if santa_turn {
                santa_pos.x -= 1;
            } else {
                robo_pos.x -= 1;
            }
        }
        
        santa_turn = !santa_turn;
        if santa_turn {
            visited_houses.insert(santa_pos.clone());
        } else {
            visited_houses.insert(robo_pos.clone());
        }
        
    }

    return visited_houses.len();

}
