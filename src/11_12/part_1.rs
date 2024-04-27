use std::collections::{HashSet, VecDeque};

use AoC_2023::text_file_reader::TextFileReader;


const GALAXIE: char = '#';
const EMPTY_SPACE: char = '.';

fn main() {
    println!("Puzzle du 11/12 Partie 1");
    
    let mut universe = get_puzzle();
    let universe_expansion_coordinates = get_universe_expansion_coordinates(&universe);

    expand_universe(&mut universe, universe_expansion_coordinates.0);
    universe = rotate_universe_90d(universe, '+');
    expand_universe(&mut universe, universe_expansion_coordinates.1);
    universe = rotate_universe_90d(universe, '-');
    let coordinates_of_galaxies = get_coordinates_of_galaxies(universe);
    let distances_between_first_and_ohters_galaxies = get_distances_between_first_and_ohters_galaxies(coordinates_of_galaxies);
    let sum = get_sum_of_the_shortest_path_between_every_pair_of_galaxies(distances_between_first_and_ohters_galaxies);
    println!("Sum of the shortest path between every pair of galaxies {sum}");    
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("11_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_universe_expansion_coordinates(universe: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
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

    let sort_and_reverse = |expansion: HashSet<_>| -> Vec<usize> {
        let mut expansion = expansion.into_iter().collect::<Vec<_>>();
        expansion.sort();
        expansion.reverse();
        expansion
    };

    (sort_and_reverse(expansion_in_y), sort_and_reverse(expansion_in_x))
}

fn expand_universe(universe: &mut Vec<String>, universe_expansion_in_one_direction: Vec<usize>) {
    let empty_universe_line = EMPTY_SPACE.to_string().repeat(universe.get(0).unwrap().len());

    for index in universe_expansion_in_one_direction {
        universe.insert(index, empty_universe_line.clone());
    }
}

fn rotate_universe_90d(universe: Vec<String>, rotation_sense: char) -> Vec<String> {
    let mut universe_lines_in_rotation: Vec<VecDeque<_>> = Vec::new();
    let is_clockwise = matches!(rotation_sense, '+');

    for universe_line in universe {
        for (index, universe_piece) in universe_line.chars().enumerate() {
            match universe_lines_in_rotation.get_mut(index) {
                None => universe_lines_in_rotation.push(vec![universe_piece].into()),
                Some(universe_line_in_rotation) => {
                    if is_clockwise {
                        universe_line_in_rotation.push_front(universe_piece);
                    } else {
                        universe_line_in_rotation.push_back(universe_piece);
                    }
                }
            }
        }
    }

    if !is_clockwise {
        universe_lines_in_rotation.reverse();
    }

    universe_lines_in_rotation
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect()
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

fn get_distances_between_first_and_ohters_galaxies(mut coordinates_of_galaxies: VecDeque<(i32, i32)>) -> VecDeque<(i32, i32)> {
    let coordinates_first_galaxy = coordinates_of_galaxies.pop_front().unwrap();

    coordinates_of_galaxies
        .into_iter()
        .map(|coordinates_of_galaxy| {
            ((coordinates_of_galaxy.0 - coordinates_first_galaxy.0), (coordinates_of_galaxy.1 - coordinates_first_galaxy.1))
        })
        .collect()
}

fn get_sum_of_the_shortest_path_between_every_pair_of_galaxies(mut distances_between_first_and_ohters_galaxies: VecDeque<(i32, i32)>) -> i32 {
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
