use AoC_2023::text_file_reader::TextFileReader;
use itertools::Itertools;

fn main() {
    println!("Puzzle du 06/12 Partie 2");
    
    let puzzle = get_puzzle();
    let time_and_distance = extract_time_and_distance(puzzle);
    let count_of_ways_to_win = get_count_of_ways_to_win(time_and_distance);
    
    println!("Nombre de façons de gagner : {count_of_ways_to_win}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("06_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn extract_time_and_distance(puzzle: Vec<String>) -> (i64, i64) {
    let mut time = 0;
    let mut distance = 0;

    for (index, p) in puzzle.iter().enumerate() {
        let value = p.split_whitespace().skip(1).collect::<Vec<_>>().iter().map(|x| x.trim()).join("").parse::<i64>().unwrap();
        if index == 0 {
            time = value;
        } else {
            distance = value;
        }
    }

    (time, distance)
}

fn get_count_of_ways_to_win((time, distance): (i64, i64)) -> i64 {
    let mut tipping_point = 0;

    for ms in 0..time {
        let possible_distance = (time - ms) * ms;
        if possible_distance > distance {
            tipping_point = ms;
            break;   
        }
    }
    let count = ((time - tipping_point) - tipping_point) + 1;

    count
}