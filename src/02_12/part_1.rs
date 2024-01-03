use AoC_2023::text_file_reader::TextFileReader;


const BLUE_STR: &str = "blue";
const RED_STR: &str = "red";
const GREEN_STR: &str = "green";

const MAX_BLUE_CUBES: u32 = 14;
const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;

fn main() {
    println!("Puzzle du 02/12 Partie 1");
    
    let puzzle = get_puzzle();
    let sum_ids = get_sum_of_valid_game_ids(puzzle);
    println!("sum of the IDs of valid games : {sum_ids}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("02_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_sum_of_valid_game_ids(puzzle: Vec<String>) -> u32 {
    let mut ids = 0;

    for p in puzzle {
        let mut game_description = p.split(":").collect::<Vec<_>>();
        game_description.reverse();
        
        let id = game_description.pop().unwrap().split_whitespace().collect::<Vec<_>>().get(1).unwrap().parse::<u32>().unwrap();
    
        let sets = game_description[0].split(";").collect::<Vec<_>>();
        if is_valid(sets) {
            ids += id;
        }
    }

    ids
}

fn is_valid(sets: Vec<&str>) -> bool {
    
    for set in sets {
        let cubes = set.split(",").collect::<Vec<_>>();
        for cube in cubes {
            let cube_data = cube.trim().split_whitespace().collect::<Vec<_>>();
            let valid_cube = match cube_data[1] {
                BLUE_STR => cube_data[0].parse::<u32>().unwrap() <= MAX_BLUE_CUBES,
                RED_STR => cube_data[0].parse::<u32>().unwrap() <= MAX_RED_CUBES,
                GREEN_STR => cube_data[0].parse::<u32>().unwrap() <= MAX_GREEN_CUBES,
                _ => true,
            };

            if !valid_cube {
                return false;
            }           
        }
    }
    true
}