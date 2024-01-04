use std::collections::HashMap;

use AoC_2023::text_file_reader::TextFileReader;


fn main() {
    println!("Puzzle du 04/12 Partie 2");
    
    let puzzle = get_puzzle();
    let total_scratchcards = get_total_scratchcards(puzzle);
    println!("Total scratchcards : {total_scratchcards}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("04_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_total_scratchcards(puzzle: Vec<String>) -> u32 {
    let mut total_scratchcards = 0;

    let mut card_id = 1;
    let mut instances_of_scratchcards = HashMap::new();

    for p in puzzle {
        let (winning_numbers, numbers_you_have) = get_numbers_lists(p);
        let winning_numbers_you_have = winning_numbers.iter().filter(|n| numbers_you_have.contains(n)).collect::<Vec<_>>();

        let mut count_of_copie = 1;
        if instances_of_scratchcards.contains_key(&card_id) {
            count_of_copie += instances_of_scratchcards.remove(&card_id).unwrap();
        }
        total_scratchcards += count_of_copie;
        
        if !winning_numbers_you_have.is_empty() {
            for i in 1..=winning_numbers_you_have.len() {
                let count = instances_of_scratchcards.entry(card_id + i as u32).or_insert(0);
                *count += 1 * count_of_copie;
            }
        }

        card_id += 1;
    }

    total_scratchcards
}

fn get_numbers_lists(scratchcards: String) -> (Vec<String>, Vec<String>) {
    let scratchcards = scratchcards.split(":").collect::<Vec<_>>();
    let mut numbers_list = scratchcards.get(1).unwrap().split(" | ").map(|s| s.to_owned()).collect::<Vec<_>>();
    
    let numbers_you_have = numbers_list.pop().unwrap();
    let winning_numbers = numbers_list.pop().unwrap();

    let numbers_you_have = numbers_you_have.split_whitespace().map(|s| s.trim().to_string()).collect::<Vec<_>>();
    let winning_numbers = winning_numbers.split_whitespace().map(|s| s.trim().to_string()).collect::<Vec<_>>();

    (winning_numbers, numbers_you_have)
}