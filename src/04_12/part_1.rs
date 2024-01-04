use AoC_2023::text_file_reader::TextFileReader;


fn main() {
    println!("Puzzle du 04/12 Partie 1");
    
    let puzzle = get_puzzle();
    let points = get_points(puzzle);
    println!("Points : {points}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("04_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_points(puzzle: Vec<String>) -> u32 {
    let mut points = 0;

    for p in puzzle {
        let (winning_numbers, numbers_you_have) = get_numbers_lists(p);
        let winning_numbers_you_have = winning_numbers.iter().filter(|n| numbers_you_have.contains(n)).collect::<Vec<_>>();

        if !winning_numbers_you_have.is_empty() {
            points += 2u32.pow(winning_numbers_you_have.len() as u32 - 1);
        }
    }


    points
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