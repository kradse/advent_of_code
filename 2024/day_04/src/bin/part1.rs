fn main() {
    let input = include_str!("./input.txt");
    let result = parse_input(input);
    dbg!(result);
}

#[test]
fn test() {
    let input = include_str!("./test.txt");
    let result = parse_input(input);
    assert_eq!(result, 18);
}

fn parse_input(input: &str) -> usize
{
    let mut word_count: usize = 0;
    word_count += count_vertical(input);
    word_count += count_horizontal(input, "XMAS");
    word_count += count_horizontal(input, "SAMX");
    word_count += count_diagonal(input, "XMAS");
    word_count += count_diagonal(input, "SAMX");
    return word_count;
}

fn count_horizontal(text: &str, word: &str) -> usize
{
    text.lines()
        .map(|line| line.matches(word).count())
        .sum()
}

fn count_vertical(text: &str) -> usize
{
    let mut word_count: usize = 0;
    let lines = transpose_text(text);
    for line in lines {
        word_count += count_horizontal(&line, "XMAS");
        word_count += count_horizontal(&line, "SAMX");
    }

    return word_count;
}

fn transpose_text(text: &str) -> Vec<String> 
{
    let rows: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let row_count = rows.len();
    let col_count = rows[0].len();
    
    (0..col_count).map(|col| {
        (0..row_count)
            .map(|row| rows[row][col])
            .collect()
    }).collect()
}

fn count_diagonal(text: &str, word: &str) -> usize {
    let grid: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let mut count = 0;

    // Check diagonals top-left to bottom-right (↘)
    for row in 0..rows {
        for col in 0..cols {
            if row + word_len <= rows && col + word_len <= cols {
                let mut match_found = true;
                for i in 0..word_len {
                    if grid[row + i][col + i] != word.chars().nth(i).unwrap() {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }

    // Check diagonals top-right to bottom-left (↙)
    for r in 0..rows {
        for c in 0..cols {
            if r + word_len <= rows && c >= word_len - 1 {
                let mut match_found = true;
                for i in 0..word_len {
                    if grid[r + i][c - i] != word.chars().nth(i).unwrap() {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }

    count
}