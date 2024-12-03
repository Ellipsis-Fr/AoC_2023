use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 06/12 Partie 1");
    
    let puzzle = get_puzzle();
    let times_and_distances = extract_time_and_distance(puzzle);
    let count_of_ways_to_win = get_count_of_ways_to_win(times_and_distances);
    
    println!("Nombre de façons de gagner : {count_of_ways_to_win}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("06_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn extract_time_and_distance(puzzle: Vec<String>) -> Vec<(i32, i32)> {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for (index, p) in puzzle.iter().enumerate() {
        let values = p.split_whitespace().skip(1).collect::<Vec<_>>().iter().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<_>>();
        if index == 0 {
            times = values;
        } else {
            distances = values;
        }
    }

    times.iter().zip(distances.iter()).map(|(&a, &b)| (a, b)).collect()
}

fn get_count_of_ways_to_win(times_and_distances: Vec<(i32, i32)>) -> i32 {
    let mut count = 1;
    
    for (time, distance) in times_and_distances {
        let mut tipping_point = 0;

        for ms in 0..time {
            let possible_distance = (time - ms) * ms;
            if possible_distance > distance {
                tipping_point = ms;
                break;   
            }
        }
        count *= ((time - tipping_point) - tipping_point) + 1;
    }

    count
}