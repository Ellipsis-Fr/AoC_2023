use std::{collections::{HashSet, VecDeque}, ops::Range};
use AoC_2023::text_file_reader::TextFileReader;


const GALAXIE: char = '#';
const EXPANSION_FACTOR: u32 = 1_000_000;

fn main() {
    println!("Puzzle du 11/12 Partie 1");
    
    let universe = get_puzzle();
    let universe_expansion_coordinates = get_universe_expansion_coordinates(&universe);

    let coordinates_of_galaxies = get_coordinates_of_galaxies(universe);
    let distances_between_first_and_ohters_galaxies = get_distances_between_first_and_ohters_galaxies(coordinates_of_galaxies, universe_expansion_coordinates);
    let sum = get_sum_of_the_shortest_path_between_every_pair_of_galaxies(distances_between_first_and_ohters_galaxies);
    println!("Sum of the shortest path between every pair of galaxies {sum}");    
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("11_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_universe_expansion_coordinates(universe: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut expansion_in_y = HashSet::new();
    let mut expansion_in_x = (0..universe.get(0).unwrap().len()).collect::<HashSet<_>>();
    
    for (index, universe_line) in universe.iter().enumerate() {
        let positions_in_column = universe_line.match_indices(GALAXIE).map(|(column, _)| column).collect::<HashSet<_>>();
        if positions_in_column.is_empty() {
            expansion_in_y.insert(index);
        } else {
            expansion_in_x.retain(|index| !positions_in_column.contains(index));
        }
    }

    let sort = |expansion: HashSet<_>| -> Vec<u32> {
        let mut expansion = expansion.into_iter().map(|x| x as u32).collect::<Vec<u32>>();
        expansion.sort();
        expansion
    };

    (sort(expansion_in_y), sort(expansion_in_x))
}

fn get_coordinates_of_galaxies(universe: Vec<String>) -> VecDeque<(i32, i32)> {
    universe
        .into_iter()
        .enumerate()
        .map(|(y, universe_line)| {
            universe_line.match_indices(GALAXIE).map(|(column, _)| column).collect::<Vec<_>>().into_iter().map(|x| (y as i32, x as i32)).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|v| v)
        .collect()
}

fn get_distances_between_first_and_ohters_galaxies(mut coordinates_of_galaxies: VecDeque<(i32, i32)>, universe_expansion_coordinates: (Vec<u32>, Vec<u32>)) -> VecDeque<(i64, i64)> {
    let coordinates_first_galaxy = coordinates_of_galaxies.pop_front().unwrap();
    let counter_of_expansion_lines = |a: i32, b: i32, universe_expansion_coordinates_in_one_direction: &Vec<u32>| -> i32 {
        let sign;
        let range;

        if a < b {
            sign = -1;
            range = Range {start: a as u32, end: b as u32};
        } else {
            sign = 1;
            range = Range {start: b as u32, end: a as u32};
        };
                
        universe_expansion_coordinates_in_one_direction
            .iter()
            .filter(|coordinate| range.contains(coordinate))
            .count() as i32 * sign
    };

    coordinates_of_galaxies
        .into_iter()
        .map(|coordinates_of_galaxy| {
            (
                (coordinates_of_galaxy.0 - coordinates_first_galaxy.0 + (counter_of_expansion_lines(coordinates_of_galaxy.0, coordinates_first_galaxy.0, &universe_expansion_coordinates.0)) * (EXPANSION_FACTOR as i32 - 1)),
                (coordinates_of_galaxy.1 - coordinates_first_galaxy.1 + (counter_of_expansion_lines(coordinates_of_galaxy.1, coordinates_first_galaxy.1, &universe_expansion_coordinates.1)) * (EXPANSION_FACTOR as i32 - 1))
            )
        })
        .map(|coordinates_of_galaxy| (coordinates_of_galaxy.0 as i64, coordinates_of_galaxy.1 as i64))
        .collect()
}

fn get_sum_of_the_shortest_path_between_every_pair_of_galaxies(mut distances_between_first_and_ohters_galaxies: VecDeque<(i64, i64)>) -> i64 {
    let mut sum = 0;

    loop {
        if let Some(distance_0) = distances_between_first_and_ohters_galaxies.pop_front() {
            sum += distance_0.0.abs() + distance_0.1.abs();

            for distance_1 in &distances_between_first_and_ohters_galaxies {
                sum += (distance_1.0 - distance_0.0).abs() + (distance_1.1 - distance_0.1).abs();
            }
        } else {
            break;
        }
    }

    sum
}
