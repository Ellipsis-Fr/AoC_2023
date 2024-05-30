use std::{collections::VecDeque, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 13/12 Partie 1");
    let now = Instant::now();
    
    let patterns = get_patterns();
    let total = get_total(patterns);
    println!("Total of summarizing all notes {total}");
    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("13_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_patterns() -> Vec<Vec<String>> {
    let puzzle = get_puzzle();
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();

    for p in puzzle.into_iter() {
        if p.is_empty() {
            patterns.push(pattern.clone());
            pattern.clear();
        } else {
            pattern.push(p);
        }
    }
    patterns.push(pattern.clone());

    patterns
}

fn get_total(patterns: Vec<Vec<String>>) -> usize {
    let mut total = 0;

    'main: for mut pattern in patterns {
        for v in 0..2 {
            let mut max = 0;
            for index in 0..(pattern.len() - 1) {
                if is_reflected(pattern.clone(), index) {
                    if index >= max {
                        max = index + 1;
                    }
                }
            }

            if max != 0 {
                total += if v == 0 { max * 100 } else { max };
                continue 'main;
            } else {
                pattern = rotate_90d(pattern);
            }
        }
        unreachable!();
    }

    total
}

fn is_reflected(pattern: Vec<String>, center: usize) -> bool {
    let mut center_1 = center;
    let mut center_2 = center + 1;
    
    loop {
        match pattern.get(center_1) {
            None => unreachable!(),
            Some(relief_1) => {
                match pattern.get(center_2) {
                    None => break,
                    Some(relief_2) => {
                        if relief_1 != relief_2 {
                            return false;
                        }
                    }                    
                }
            }
        }

        if let Some(value) = center_1.checked_sub(1) {
            center_1 = value;
        } else {
            break;
        }
        center_2 += 1;
    }
    true
}

fn rotate_90d(pattern: Vec<String>) -> Vec<String> {
    let pattern_chars = pattern.into_iter().map(|p| p.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut pattern_chars_rotated = Vec::new();

    for chars in pattern_chars {
        for (index, char) in chars.into_iter().enumerate() {
            match pattern_chars_rotated.get_mut(index) {
                None => {
                    let mut new_order = VecDeque::new();
                    new_order.push_front(char);
                    pattern_chars_rotated.push(new_order);
                },
                Some(new_order) => new_order.push_front(char)
            }
        }
    }

    pattern_chars_rotated.into_iter().map(|c| c.into_iter().collect::<String>()).collect()
}