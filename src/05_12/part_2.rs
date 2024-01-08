use std::ops::Range;

use AoC_2023::text_file_reader::TextFileReader;
use itertools::Itertools;

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
    println!("Puzzle du 05/12 Partie 2");
    
    let puzzle = get_puzzle();
    let (seeds, plantation_stages) = get_data(puzzle);
    let lowest_location = get_lowest_location(seeds, plantation_stages, 0);
    println!("Lowest location : {lowest_location}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("05_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_data(puzzle: Vec<String>) -> (Vec<Range<i64>>, Vec<Vec<(i64, i64, i64)>>) {
    let mut seeds_data = Vec::new(); // seeds initial localisation
    let mut planting_data = Vec::new(); // [[(source_start, shift, diff with destination), ...], ...]
    let mut planting_data_by_stage = Vec::new(); // (source_start, shift, diff with destination)

    
    for p in puzzle {
        if p.contains(SEEDS_STR) {
            let seeds_localisation = p.split(":").collect::<Vec<_>>();
            let seeds_localisation = seeds_localisation[1].trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
            seeds_data = seeds_localisation.iter().tuples().map(|(start, length)| *start..(start + length - 1)).collect();
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

fn get_lowest_location(seeds: Vec<Range<i64>>, plantation_stages: Vec<Vec<(i64, i64, i64)>>, to_skip: usize) -> i64 {
    let mut lowest_location = i64::MAX;
    let find_where_to_plant = |seed_range_location: &Range<i64>, source_start: i64, shift: i64| -> bool { ranges_cross(seed_range_location, &(source_start..(source_start + shift + 1))) };

    for mut seed_range_location in seeds {
        'outer: for (index, plantation_stage) in plantation_stages.iter().enumerate().skip(to_skip) {
            let mut possible_locations_where_to_plant = plantation_stage.iter().filter(|(source_start, shift, _)| find_where_to_plant(&seed_range_location, *source_start, *shift)).collect::<Vec<_>>();

            if possible_locations_where_to_plant.is_empty() {
                continue;
            } else if possible_locations_where_to_plant.len() == 1 {
                let (source_start, shift, diff) = possible_locations_where_to_plant.pop().unwrap();
                let mut remaining_seeds = get_remaining_seeds(vec![seed_range_location.clone()], &(*source_start..(*source_start + *shift)), false);

                if seed_range_location.start < *source_start && seed_range_location.end > *source_start + *shift {
                    seed_range_location.start = *source_start;
                    seed_range_location.end = *source_start + *shift;
                } else if seed_range_location.start < *source_start {
                    seed_range_location.start = *source_start;
                } else if seed_range_location.end > *source_start + *shift {
                    seed_range_location.end = *source_start + *shift;
                }

                seed_range_location.start += *diff;
                seed_range_location.end += *diff;

                
                if !remaining_seeds.is_empty() {
                    remaining_seeds = get_remaining_seeds(remaining_seeds, &seed_range_location, true);
                    let mut new_seeds = vec![seed_range_location.clone()];
                    new_seeds.append(&mut remaining_seeds);
                    seed_range_location.start = get_lowest_location(new_seeds, plantation_stages.clone(), index + 1);
                    break 'outer;
                } 
            } else {
                let mut new_seeds = Vec::new();
                let mut remaining_seeds = Vec::new();
                for (source_start, shift, diff) in possible_locations_where_to_plant {

                    // ! Ce cas est différent du traitement unitaire, car le reste devient l'élément qui sera à observer ensuite du point de vue des autres zones de plantations possibles...
                    remaining_seeds = if remaining_seeds.is_empty() {
                        get_remaining_seeds(vec![seed_range_location.clone()], &(*source_start..(*source_start + *shift)), false)
                    } else {
                        get_remaining_seeds(remaining_seeds, &(*source_start..(*source_start + *shift)), true)
                    };

                    let mut new_seed_range_location = seed_range_location.clone();
                    if new_seed_range_location.start < *source_start && new_seed_range_location.end > *source_start + *shift {
                        new_seed_range_location.start = *source_start;
                        new_seed_range_location.end = *source_start + *shift;
                    } else if new_seed_range_location.start < *source_start {
                        new_seed_range_location.start = *source_start;
                    } else if new_seed_range_location.end > *source_start + *shift {
                        new_seed_range_location.end = *source_start + *shift;
                    }
                    
                    // ! Attention en appliquant le différentiel, peut être que les nouveaux ranges créés se superposent avec les nouvelles limites de celui édité...
                    new_seed_range_location.start += *diff;
                    new_seed_range_location.end += *diff;

                    new_seeds.push(new_seed_range_location);
                }

                for new_seed_range_location in &new_seeds {
                    if !remaining_seeds.is_empty() {
                        remaining_seeds = get_remaining_seeds(remaining_seeds, new_seed_range_location, true);
                    }
                }

                if !remaining_seeds.is_empty() {
                    new_seeds.append(&mut remaining_seeds);
                    seed_range_location.start = get_lowest_location(new_seeds, plantation_stages.clone(), index + 1);
                } else {
                    seed_range_location.start = get_lowest_location(new_seeds, plantation_stages.clone(), index + 1);
                }

                break 'outer;
            }
        }

        if seed_range_location.start < lowest_location {
            lowest_location = seed_range_location.start;
        }      
    }

    lowest_location
}

fn get_remaining_seeds(initial_seeds: Vec<Range<i64>>, plantations_row: &Range<i64>, is_distinct: bool) -> Vec<Range<i64>> {
    let mut remaining_seeds = Vec::new();

    for initial_seed in initial_seeds {
        if initial_seed.start < plantations_row.start && initial_seed.end > plantations_row.end {
            let new_seed_range_location_start = initial_seed.start..(plantations_row.start - 1);
            let new_seed_range_location_end = (plantations_row.end + 1)..initial_seed.end;
            remaining_seeds.push(new_seed_range_location_start);
            remaining_seeds.push(new_seed_range_location_end);
        } else if initial_seed.start < plantations_row.start {
            let new_seed_range_location_start;
            if is_distinct {
                if initial_seed.end > plantations_row.start {
                    new_seed_range_location_start = initial_seed.start..(plantations_row.start - 1);
                } else {
                    new_seed_range_location_start = initial_seed;
                }
            } else {
                new_seed_range_location_start = initial_seed.start..(plantations_row.start - 1);
            }
            remaining_seeds.push(new_seed_range_location_start);
        } else if initial_seed.end > plantations_row.end {
            let new_seed_range_location_end;
            if is_distinct {
                if initial_seed.start < plantations_row.end {
                    new_seed_range_location_end = (plantations_row.end + 1)..initial_seed.end;
                } else {
                    new_seed_range_location_end = initial_seed;
                }
            } else {
                new_seed_range_location_end = (plantations_row.end + 1)..initial_seed.end;
            }
            remaining_seeds.push(new_seed_range_location_end);
        }
    }

    remaining_seeds
}

fn ranges_cross<T: PartialOrd>(a: &Range<T>, b: &Range<T>) -> bool {
    if b.start < a.start {
        a.start <= b.end
    } else if b.start > a.start {
        a.end >= b.start
    } else {
        true
    }
}