use std::{collections::VecDeque, ops::RangeInclusive, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 13/12 Partie 1");
    let now = Instant::now();
    
    let patterns = get_patterns();
    // for p in patterns {
        
    //     let a = rotate_90d(p);
    //     print(a);
    //     println!();
    // }
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
        let pattern_in_error = pattern.clone();
        for v in 0..2 {
            // for center in get_range_middle(pattern.len()) {
            for center in 0..pattern.len() {
                if is_center(pattern.clone(), center) {
                    // println!("center : {center}");
                    if v == 0 {
                        total += (center + 1) * 100;
                    } else {
                        total += center + 1;
                    }
                    continue 'main;
                }
            }
            pattern = rotate_90d(pattern);
        }
        print(pattern_in_error);
        println!();
        println!();
        print(pattern);
        panic!("no mirror found");
    }

    total
}

fn get_range_middle(len: usize) -> RangeInclusive<usize> {
    let middle = len / 2;

    if len % 2 == 0 {
        (middle - 1)..=middle
    } else {
        (middle - 1)..=(middle + 1)
    }
}

fn is_center(pattern: Vec<String>, center: usize) -> bool {
    let mut center_1 = center;
    let mut center_2 = center + 1;
    
    loop {
        match pattern.get(center_1) {
            None => break,
            Some(relief_1) => {
                match pattern.get(center_2) {
                    None => break,
                    Some(relief_2) => {
                        if relief_1 != relief_2 {
                            return false;
                        }
                        if center_1 == 0 {
                            break;
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

fn print(pattern: Vec<String>) {
    pattern.iter().for_each(|p| {
        println!("{p}");
    });
}