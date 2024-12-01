fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

struct Cube {
    length: usize,
    width: usize,
    height: usize,
}

fn parse_input(input: &str) -> usize {
    let mut total_sum: usize = 0;

    input.lines()
         .for_each(|line| {
            let mut dimensions: Vec<usize> = line.split('x')
                                             .map(|no| no.parse::<usize>().unwrap())
                                             .collect();

            let cube = Cube {
                length: dimensions[0],
                width: dimensions[1],
                height: dimensions[2],
            };

            dimensions.sort_unstable();

            let ribbon: usize = (dimensions[0] * 2) + (dimensions[1] * 2);
            let bow: usize = cube.length * cube.width * cube.height;
            let sum = ribbon + bow;

            total_sum += sum;
        }
    );
    
    return total_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = parse_input(input);
        assert_eq!(result, 48);
    }
}