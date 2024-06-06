use std::time::Instant;

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 15/12 Partie 1");
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
    sequences.iter().map(|sequence| do_hash(sequence)).sum()
}

fn do_hash(sequence: &String) -> u32 {
    let mut hash = 0;
    sequence.chars().into_iter().for_each(|c| hash = ((hash + (c as u32)) * 17) % 256);
    hash
}