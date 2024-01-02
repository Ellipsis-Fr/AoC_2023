use AoC_2023::text_file_reader::TextFileReader;


fn main() {
    println!("Puzzle du 01/12 Partie 1");
    
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
    let mut calibration = 0;

    for p in puzzle {
        let numbers = p.chars().into_iter().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let ten = numbers.get(0).unwrap().clone() * 10;
        let unit = numbers.get(numbers.len() - 1).unwrap().clone();

        calibration += ten + unit;
    }

    calibration
}