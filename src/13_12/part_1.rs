use std::time::Instant;

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 13/12 Partie 1");
    let now = Instant::now();
    
    let damaged_records = get_puzzle();
    println!("took: {:?}", now.elapsed());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("13_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}