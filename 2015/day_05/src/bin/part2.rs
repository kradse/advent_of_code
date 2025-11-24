fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();

    let mut no_of_nice_strings = 0;
    for line in lines {

        let mut rule_1 = false;
        let mut rule_2 = false;

        let chars: Vec<char> = line.chars().collect();
        for i in 1..chars.len() {

            let current_pair = format!("{}{}", chars[i-1], chars[i-0]);
            let count = line.match_indices(&current_pair).count();
            if count == 2 {
                rule_1 = true;
            }
            
            if i >= 2 {
                let char = chars[i];
                let check_char = chars[i-2];
                if char == check_char {
                    rule_2 = true;
                }
            }
        }

        if (rule_1) && (rule_2) {
            no_of_nice_strings += 1;
        }
    }

    println!("No. of nice strings: {:?}", no_of_nice_strings);
}
