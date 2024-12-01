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

fn parse_input(input: &str) -> usize
{
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    
    input.lines().for_each(|line| {
            let row: Vec<usize> = line.split("   ")
                                      .map(|id| id.parse::<usize>().unwrap())
                                      .collect();

            left.push(row[0]);
            right.push(row[1]);           
        }
    );

    left.sort_unstable();
    right.sort_unstable();

    let mut total_distance: usize = 0;

    for i in 0..left.len() {
        total_distance += left[i].abs_diff(right[i]);        
    }
    
    return total_distance;
}