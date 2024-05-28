use std::{ops::{Range, RangeInclusive}, time::Instant};

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 13/12 Partie 1");
    let now = Instant::now();
    
    let patterns = get_patterns();
    let total = get_total(patterns);
    println!("Total of summarizing all notes{total}");
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
            for center in get_range_middle(pattern.len()) {
                if is_center(pattern.clone()) {
                    if v == 0 {
                        total += center * 100;
                    } else {
                        total += center;
                    }
                    continue 'main;
                }
            }
            pattern = rotate_90d(pattern);
        }
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

fn is_center(pattern: Vec<String>) -> bool {
    todo!()
}

fn rotate_90d(pattern: Vec<String>) -> Vec<String> {
    todo!()
}