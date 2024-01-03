use AoC_2023::text_file_reader::TextFileReader;


const BLUE_STR: &str = "blue";
const RED_STR: &str = "red";
const GREEN_STR: &str = "green";


fn main() {
    println!("Puzzle du 02/12 Partie 2");
    
    let puzzle = get_puzzle();
    let sum_ids = get_sum_of_power_sets(puzzle);
    println!("sum of the IDs of valid games : {sum_ids}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("02_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_sum_of_power_sets(puzzle: Vec<String>) -> u32 {
    let mut power_sets = 0;

    for p in puzzle {
        let sets = p.split(":").collect::<Vec<_>>().pop().unwrap().split(";").collect::<Vec<_>>();
        power_sets += get_power_set(sets);
    }

    power_sets
}

fn get_power_set(sets: Vec<&str>) -> u32 {
    let mut min_blue_cubes = 0;
    let mut min_red_cubes = 0;
    let mut min_green_cubes = 0;

    for set in sets {
        let cubes = set.split(",").collect::<Vec<_>>();
        for cube in cubes {
            let cube_data = cube.trim().split_whitespace().collect::<Vec<_>>();
            let cubes_number = cube_data[0].parse::<u32>().unwrap();

            match cube_data[1] {
                BLUE_STR => {
                    if cubes_number > min_blue_cubes {
                        min_blue_cubes = cubes_number;
                    }
                },
                RED_STR => {
                    if cubes_number > min_red_cubes {
                        min_red_cubes = cubes_number;
                    }
                },
                GREEN_STR => {
                    if cubes_number > min_green_cubes {
                        min_green_cubes = cubes_number;
                    }
                },
                _ => (),
            }          
        }
    }


    min_blue_cubes * min_red_cubes * min_green_cubes
}