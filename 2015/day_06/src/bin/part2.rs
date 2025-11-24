use regex::Regex;

fn main() {
    let mut light_grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut brightness: usize = 0;

    let input = include_str!("./input.txt");
    let lines = input.lines();

    for line in lines {
        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = 0;
        let mut y2: usize = 0;

        let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();

        if let Some(caps) = re.captures(line) {
            x1 = caps[1].parse().unwrap();
            y1 = caps[2].parse().unwrap();
            x2 = caps[3].parse().unwrap();
            y2 = caps[4].parse().unwrap();
        }

        if line.contains("turn on") {
            for row in y1..=y2 {
                for bulb in x1..=x2 {
                    light_grid[row][bulb] += 1;
                }
            }
        }
        if line.contains("turn off") {
            for row in y1..=y2 {
                for bulb in x1..=x2 {
                    if light_grid[row][bulb] > 0 {
                        light_grid[row][bulb] -= 1;
                    }
                }
            }
        }
        if line.contains("toggle") {
            for row in y1..=y2 {
                for bulb in x1..=x2 {
                    light_grid[row][bulb] += 2;
                }
            }
        }
    }

    for row in light_grid {
        for bulb in row {
            brightness += bulb;
        }
    }

    println!("Total brightness: {:?}", brightness);
}

