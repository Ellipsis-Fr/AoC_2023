use std::{collections::{HashMap, VecDeque}, time::Instant};

use itertools::Itertools;
use regex::Regex;
use AoC_2023::text_file_reader::TextFileReader;

const EQUALS_SIGN: &str = "=";
const DASH_SIGN: &str = "-";

fn main() {
    println!("Puzzle du 15/12 Partie 2");
    let now = Instant::now();
    
    let initialization_sequence  = get_puzzle();
    let sum = compute_sequence(initialization_sequence[0].split(",").map(|s| s.to_string()).collect::<Vec<_>>());
    println!("Sum: {sum}");
    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("15_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn compute_sequence(sequences: Vec<String>) -> u32 {
    let mut sum = 0;
    let boxes = construct_sequence(sequences);
    
    for (index, box_) in boxes.into_iter().enumerate() {
        for (index_box, lens_slot) in box_.iter().enumerate() {
            let focal_length = lens_slot.split_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap();
            sum += (index as u32 + 1) * (index_box as u32 + 1) * focal_length;
        }
    }

    sum
}

fn construct_sequence(sequences: Vec<String>) -> Vec<Vec<String>> {
    let mut boxes: Vec<Vec<String>> = Vec::new();

    for sequence in &sequences {
        let operation = get_operation(sequence);
        let lens_slot = split_sequence(sequence, operation);
        let label =  &lens_slot.split_whitespace().collect::<Vec<_>>()[0];
        let hash = do_hash(label);

        match boxes.get_mut(hash) {
            Some(box_) => {
                if box_.iter().filter(|seq| contains_regex(seq, label)).count() > 0 {
                    
                    let index = {
                        let temp = box_.iter().find_position(|seq| contains_regex(seq, label)).unwrap();
                        temp.0
                    };
                    
                    if operation == EQUALS_SIGN {
                        if let Some(lens_slots) = box_.get_mut(index) {
                            *lens_slots = lens_slot;
                        }
                    } else {
                        box_.remove(index);
                        box_.retain(|s| !s.is_empty());
                    }
                } else {
                    if operation == EQUALS_SIGN {
                        box_.push(lens_slot);
                    }
                }
            },
            None => {
                if operation == EQUALS_SIGN {
                    let mut box_ = Vec::new();
                    box_.push(lens_slot);

                    if hash > boxes.len() {
                        let resize = boxes.len() + (hash + 1) - boxes.len();
                        boxes.resize(resize, Vec::new());
                    }

                    boxes.insert(hash, box_);
                }
            }
        }
    }

    boxes
}

fn do_hash(label: &str) -> usize {
    let mut hash = 0;
    label.chars().into_iter().for_each(|c| hash = ((hash + (c as usize)) * 17) % 256 );
    hash
}

fn get_operation(sequence: &String) -> &str {
    if sequence.contains(EQUALS_SIGN) {
        EQUALS_SIGN
    } else {
        DASH_SIGN
    }
}

fn split_sequence(sequence: &String, operation: &str) -> String {
    let split = sequence.split(operation).collect::<Vec<&str>>();
    split.join(" ")
}

fn contains_regex(sequence: &String, label: &str) -> bool {
    let regex = Regex::new(format!("^{}", &label).as_str()).unwrap();
    regex.is_match(&sequence)
}