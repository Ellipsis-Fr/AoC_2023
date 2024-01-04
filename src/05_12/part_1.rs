use AoC_2023::text_file_reader::TextFileReader;

const SEEDS_STR: &str = "seeds";
const SEEDS_TO_SOIL_STR: &str = "seed-to-soil";
const SOIL_TO_FERTILIZER_STR: &str = "soil-to-fertilizer";
const FERTILIZER_TO_WATER_STR: &str = "fertilizer-to-water";
const WATER_TO_LIGHT_STR: &str = "water-to-light";
const LIGHT_TO_TEMPERATURE_STR: &str = "light-to-temperature";
const TEMPERATURE_TO_HUMIDITY_STR: &str = "temperature-to-humidity";
const HUMIDITY_TO_LOCATION_STR: &str = "humidity-to-location";
const PLANTING_STAGES: [&str; 7] = [SEEDS_TO_SOIL_STR, SOIL_TO_FERTILIZER_STR, FERTILIZER_TO_WATER_STR, WATER_TO_LIGHT_STR, LIGHT_TO_TEMPERATURE_STR, TEMPERATURE_TO_HUMIDITY_STR, HUMIDITY_TO_LOCATION_STR];

fn main() {
    println!("Puzzle du 05/12 Partie 1");
    
    let puzzle = get_puzzle();
    let (seeds, plantation_stages) = get_data(puzzle);
    let lowest_location = get_lowest_location(seeds, plantation_stages);
    println!("Lowest location : {lowest_location}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("05_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_data(puzzle: Vec<String>) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let mut seeds_data = Vec::new(); // seeds initial localisation
    let mut planting_data = Vec::new(); // [[(source_start, shift, diff with destination), ...], ...]
    let mut planting_data_by_stage = Vec::new(); // (source_start, shift, diff with destination)

    
    for p in puzzle {
        if p.contains(SEEDS_STR) {
            let seeds_localisation = p.split(":").collect::<Vec<_>>();
            seeds_data = seeds_localisation[1].trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        } else if p.is_empty() {

        } else if PLANTING_STAGES.iter().any(|s| p.contains(s)) {
            if !planting_data_by_stage.is_empty() {
                planting_data.push(planting_data_by_stage);
                planting_data_by_stage = Vec::new();
            } 
        } else {
            let initial_planting_data = p.split_whitespace().collect::<Vec<_>>();
            let destination_start = initial_planting_data.get(0).unwrap().to_owned().parse::<i64>().unwrap();
            let source_start = initial_planting_data.get(1).unwrap().to_owned().parse::<i64>().unwrap();
            let shift = initial_planting_data.get(2).unwrap().to_owned().parse::<i64>().unwrap() - 1;

            planting_data_by_stage.push((source_start, shift, destination_start - source_start));        
        }
    }

    planting_data.push(planting_data_by_stage);

    (seeds_data, planting_data)
}

fn get_lowest_location(seeds: Vec<i64>, plantation_stages: Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut lowest_location = i64::MAX;
    let find_where_to_plant = |seed_location: i64, source_start: i64, shift: i64| -> bool { seed_location >= source_start && seed_location <= (source_start + shift) };

    for mut seed_location in seeds {
        for plantation_stage in &plantation_stages {
            match plantation_stage.iter().find(|(source_start, shift, _)| find_where_to_plant(seed_location, *source_start, *shift)) {
                None => (),
                Some((_, _, diff)) => {
                    seed_location += *diff;
                }
            }
        }

        if seed_location < lowest_location {
            lowest_location = seed_location;
        }
    }

    lowest_location
}