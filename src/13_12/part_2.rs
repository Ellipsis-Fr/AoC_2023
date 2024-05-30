use std::{collections::VecDeque, time::Instant, usize};

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 13/12 Partie 2");
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
        let mut can_be_smudged = true;
        for v in 0..2 {
            let mut max = 0;

            for index in 0..(pattern.len() - 1) {
                if is_reflected(&mut pattern, index, &mut can_be_smudged) {
                    if index >= max {
                        max = index + 1;
                    }
                    if !can_be_smudged {
                        break;
                    }
                }
            }

            if max != 0 && !can_be_smudged {
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

fn is_reflected(pattern: &mut Vec<String>, center: usize, can_be_smudged: &mut bool) -> bool {
    let mut center_1 = center;
    let mut center_2 = center + 1;
    
    let mut try_to_fix = false;
    let mut index_to_rollback = 0;
    let mut relief_to_rollback = String::new();
    
    loop {
        match pattern.get(center_1) {
            None => unreachable!(),
            Some(relief_1) => {
                match pattern.get(center_2) {
                    None => break,
                    Some(relief_2) => {
                        if relief_1 != relief_2 {
                            if *can_be_smudged {
                                let relief_differences = relief_1.char_indices().into_iter().zip(relief_2.char_indices().into_iter()).filter(|((_, c1), (_, c2))| c1 != c2).map(|((index, char), _)| (index, char)).collect::<Vec<_>>();
                                if relief_differences.len() > 1 {
                                    return false;
                                }
                                *can_be_smudged = false;
                                try_to_fix = true;

                                let (index, relief) = find_pattern_to_replace(pattern, (center_1, relief_1), (center_2, relief_2), relief_differences[0]);
                                index_to_rollback = index;
                                relief_to_rollback = if index == center_1 { relief_1.clone() } else { relief_2.clone() };                      

                                pattern.remove(index);
                                pattern.insert(index, relief);
                            } else {
                                if try_to_fix {
                                    *can_be_smudged = true;
                                    pattern.remove(index_to_rollback);
                                    pattern.insert(index_to_rollback, relief_to_rollback);  
                                }
                                return false;
                            }
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

fn find_pattern_to_replace(
    pattern: &Vec<String>,
    (center_1,relief_1): (usize, &String),
    (center_2, relief_2): (usize, &String), 
    (index, relief_1_to_change): (usize, char)
) -> (usize, String) {
    let (relief_1_to_replace, relief_2_to_replace) = if relief_1_to_change == '.' { ("#", ".") } else { (".", "#") };
    
    let mut relief_1 = relief_1.clone();
    let mut relief_2 = relief_2.clone();
    relief_1.replace_range(index..(index + 1), relief_1_to_replace);
    relief_2.replace_range(index..(index + 1), relief_2_to_replace);

    if pattern.iter().filter(|p| p == &&relief_1).count() > 1 {
        (center_2, relief_2)
    } else {
        (center_1, relief_1)
    } 
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