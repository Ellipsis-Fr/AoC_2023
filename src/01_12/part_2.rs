use std::collections::HashMap;


use AoC_2023::text_file_reader::TextFileReader;
use itertools::Itertools;


fn main() {
    println!("Puzzle du 01/12 Partie 2");
    
    let puzzle = get_puzzle();
    let calibration = read_calibration(puzzle);
    println!("{calibration}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("01_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn read_calibration(puzzle: Vec<String>) -> u32 {
    let digits_str = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibration = 0;
    
    
    for mut p in puzzle {
        if digits_str.iter().any(|s| p.contains(s)) {
            let mut theoric_digits_str_position_in_p = HashMap::new();
            
            digits_str.iter().enumerate().for_each(|(i, s)| {
                if p.contains(s) {
                    let indexes: Vec<_> = p.match_indices(s).map(|(i, _)| i).collect();
                    for index in indexes {
                        theoric_digits_str_position_in_p.insert(index, i + 1);
                    }
                }
            });

            for (index, digit) in theoric_digits_str_position_in_p.iter().sorted().rev() {
                p.insert_str(*index, &digit.to_string());
            }
        }
        let numbers = p.chars().into_iter().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let ten = numbers.get(0).unwrap().clone() * 10;
        let unit = numbers.get(numbers.len() - 1).unwrap().clone();

        calibration += ten + unit;
    }

    calibration
}