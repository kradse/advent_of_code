#[allow(unused_assignments)]
fn main() {
    // let input = include_str!("./test.txt");
    let input = include_str!("./input.txt");
    let lines = input.lines();

    let mut safe_dial = 50;
    let mut zero_counter = 0;

    for line in lines {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let value: i32 = chars.as_str().parse().unwrap();

        for _ in 0..value {
            if direction == 'R' {
                safe_dial = (safe_dial + 1) % 100;
            } else {
                safe_dial = (safe_dial + 99) % 100; // -1 mod 100
            }

            if safe_dial == 0 {
                zero_counter += 1;
            }
        }
    }

    println!("zeroes {:?}", zero_counter);
}
// 5941