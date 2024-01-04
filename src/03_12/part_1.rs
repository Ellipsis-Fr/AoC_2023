use AoC_2023::text_file_reader::TextFileReader;


fn main() {
    println!("Puzzle du 03/12 Partie 1");
    
    let puzzle = get_puzzle();
    let sum_of_part_number = get_sum_of_part_number(puzzle);
    println!("Sum of part number : {sum_of_part_number}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("03_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_sum_of_part_number(puzzle: Vec<String>) -> u32 {
    let mut sum = 0;

    let data = get_data(puzzle);
    for (index, d) in data.iter().enumerate() {
        if d.iter().any(|x| x.0 == "*") {
            sum += get_parts_number(index, &data).iter().sum::<u32>();
        }
    }

    sum
}

fn get_data(puzzle: Vec<String>) -> Vec<Vec<(String, (usize, usize))>> {
    let mut data = Vec::new();
    for p in puzzle {
        data.push(extract_data_from_each_row(p));
    }
    data
}

fn extract_data_from_each_row(p: String) -> Vec<(String, (usize, usize))> {
    let mut data = Vec::new();
    let mut number = String::new();

    for (index, c) in p.char_indices() {
        if c == '.' {
            if !number.is_empty() {
                data.push((number.clone(), (index - number.len(), index - 1)));
                number = String::new();
            }
            continue;
        } else {
            if c.is_numeric() {
                number += &c.to_string();
            } else {
                if !number.is_empty() {
                    data.push((number.clone(), (index - number.len(), index - 1)));
                    number = String::new();
                }
                data.push(("*".to_string(), (index, index)));
            }
        }
    }

    if !number.is_empty() {
        data.push((number.clone(), (p.len() - number.len(), p.len() - 1)));
    }

    data
}

fn get_parts_number(row: usize, data: &Vec<Vec<(String, (usize, usize))>>) -> Vec<u32> {
    let mut parts_number = Vec::new();

    let position_of_symbols = data.get(row).unwrap().iter().filter(|x| x.0 == "*").map(|(_, (x, _))| x).collect::<Vec<_>>();

    for index in (row - 1)..=(row + 1) {
        match data.get(index) {
            None => continue,
            Some(numbers_and_position) => {
                for (number_str, position) in numbers_and_position {
                    if number_str == "*" {
                        continue;
                    }
                    if position_of_symbols.iter().any(|x| (position.0 as i32 - **x as i32).abs() <= 1 || (position.1 as i32 - **x as i32).abs() <= 1) {
                        parts_number.push(number_str.parse().unwrap());
                    }
                }
            }
        }
    }

    parts_number
}