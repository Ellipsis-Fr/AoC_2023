use std::collections::HashMap;

use AoC_2023::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 08/12 Partie 1");
    
    let puzzle = get_puzzle();
    let (instructions, nodes) = extract_data(puzzle);
    let steps = count_step_to_go_out(instructions, nodes);
    println!("Steps to go out : {steps}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("08_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn extract_data(puzzle: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let instructions = puzzle.get(0).unwrap().to_owned();
    let mut nodes = HashMap::new();

    for p in puzzle.into_iter().skip(2) {
        let source_destinations = p.split("=").collect::<Vec<_>>().iter().cloned().map(|x| x.trim().to_string()).collect::<Vec<_>>();
        let (source_tab, destinations_tab) = source_destinations.split_at(1);
        let destinations = destinations_tab[0].clone().chars().into_iter().filter(|c| *c != '(' && *c != ')' && *c != ',').collect::<String>();
        let destinations = destinations.split_whitespace().collect::<Vec<_>>();
        let (left_destination, right_destination) = (destinations.get(0).unwrap().to_string(), destinations.get(1).unwrap().to_string());
        nodes.insert(source_tab[0].clone(), (left_destination, right_destination));
    }
    
    (instructions, nodes)
}

fn count_step_to_go_out(instructions: String, nodes: HashMap<String, (String, String)>) -> u32 {
    const INPUT_NAME: &str = "AAA";
    const OUTPUT_NAME: &str = "ZZZ";
    let mut current_position_name = INPUT_NAME;
    let mut steps = 0;
    
    'outer: loop {
        for instruction in instructions.chars() {
            current_position_name = match instruction {
                'L' => &nodes.get(current_position_name).unwrap().0,
                'R' => &nodes.get(current_position_name).unwrap().1,
                _ => panic!("unrecognized instruction")
            };

            steps += 1;

            if current_position_name == OUTPUT_NAME {
                break 'outer;
            }
        }
    }

    steps
}