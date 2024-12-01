fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

#[test]
fn test() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    assert_eq!(result, 31);
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

    let mut total_score: usize = 0;
    
    for x in &left {
        let mut similarity_score: usize = 0;
        for y in &right {
            if x == y {
                similarity_score += 1;
            }
        }

        total_score += x * similarity_score;

    }

    // left.sort_unstable();
    // right.sort_unstable();

    // let mut distance: Vec<usize> = Vec::new();
    // let mut total_distance: usize = 0;

    // for i in 0..left.len() {
    //     // println!("Left: {} - Right: {}", left[i], right[i]);
    //     // let difference = left[i].abs_diff(right[i]);
    //     total_distance += left[i].abs_diff(right[i]);
    //     // println!("difference is: {}", difference);


    //     // distance.push(math);

        
    // }

    
    return total_score;
}