use AoC_2023::text_file_reader::TextFileReader;
use itertools::Itertools;

fn main() {
    println!("Puzzle du 09/12 Partie 2");
    
    let puzzle = get_puzzle();
    let histories = puzzle.iter().map(|p| p.split_whitespace().collect::<Vec<_>>().iter().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let prediction_values = get_prediction_values(histories);
    println!("Extrapoled value : {}", prediction_values.iter().sum::<i32>())
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("09_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}


fn get_prediction_values(histories: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prediction_values = Vec::new();

    for mut sequence in histories {
        let mut first_values = vec![*sequence.first().unwrap()];
        let mut index = 0;

        loop {
            index +=1;
            sequence = sequence.iter().tuple_windows().map(|(a, b)| (b - a)).collect();
            if sequence.iter().all(|x| *x == 0) {
                break;
            }

            let mut first_value = *sequence.first().unwrap();
            first_value *= if index % 2 == 0 {
                1
            } else {
                -1
            };

            first_values.push(first_value);
        }
        
        prediction_values.push(first_values.iter().sum());
    }

    prediction_values
}