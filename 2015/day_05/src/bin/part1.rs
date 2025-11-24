fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();

    let mut no_of_nice_strings = 0;
    for line in lines {
        if line.contains("ab") {
            continue;
        }
        if line.contains("cd") {
            continue;
        }
        if line.contains("pq") {
            continue;
        }
        if line.contains("xy") {
            continue;
        }

        let mut last_char: char = '_';
        let mut double_char = false;
        let mut no_of_vowels = 0;
        for char in line.chars() {
            if char == last_char {
                double_char = true;
            }
            if matches!(char, 'a' | 'e' | 'i' | 'o' | 'u') {
                no_of_vowels += 1;
            }
            last_char = char;
        }

        if (double_char == true) && (no_of_vowels >= 3) {
            no_of_nice_strings += 1;
        }
    }

    println!("No. of nice strings: {:?}", no_of_nice_strings);
}
