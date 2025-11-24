use regex::Regex;

fn main() {
    let mut light_grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    let mut no_of_lights = 0;

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
                    light_grid[row][bulb] = true;
                }
            }
        }
        if line.contains("turn off") {
            for row in y1..=y2 {
                for bulb in x1..=x2 {
                    light_grid[row][bulb] = false;
                }
            }
        }
        if line.contains("toggle") {
            for row in y1..=y2 {
                for bulb in x1..=x2 {
                    let light = light_grid[row][bulb];
                    light_grid[row][bulb] = !light;
                }
            }
        }
    }
    
    for row in light_grid {
        for bulb in row {
            if bulb {
                no_of_lights += 1;
            }
        }
    }

    println!("No. of lights turned on: {:?}", no_of_lights);
}
