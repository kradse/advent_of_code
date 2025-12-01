#[allow(unused_assignments)]
fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();

    let mut safe_dial = 50;
    let mut zero_counter = 0;

    for line in lines {
        let mut turn_left = false;
        let mut turn_right = false;

        let mut value_int = 0;
        let mut value_str: String = "".to_string();
        
        let mut i = 0;
        for char in line.chars() {
            if i == 0 {
                if char == 'L' {
                    turn_left = true;
                    turn_right = false;
                }
                if char == 'R' {
                    turn_left = false;
                    turn_right = true;
                }
            } else {
                value_str += &char.to_string();
            }

            i += 1;
        }

        value_int = value_str.parse::<isize>().unwrap();

        if turn_left {
            safe_dial -= value_int;
            while safe_dial < 0 {
                safe_dial += 100;
            }
            if safe_dial == 0 {
                zero_counter += 1;
            }
        }
        if turn_right {
            safe_dial += value_int;
            while safe_dial > 99 {
                safe_dial -= 100;
            }
            if safe_dial == 0 {
                zero_counter += 1;
            }
        }
    }
    println!("final value {:?}", safe_dial);
    println!("{:?}", zero_counter);
}
// 989