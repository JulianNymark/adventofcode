use std::fs;

fn main() {
    part_1("input.txt");
}

fn part_1(file: &str) {
    let file_contents = fs::read_to_string(file).unwrap();

    let mut total = 0;

    for line in file_contents.lines() {
        let result = get_result(line);
        println!("result: {}", result);
        total += result;
    }

    println!("total: {}", total);
}

// returns 0 if not valid, otherwise, the idx from line
fn get_result(line: &str) -> i32 {
    let game: Vec<&str> = line.split(":").collect();
    let game_id = game[0].replace("Game ", "").parse::<i32>().unwrap();

    let rounds: Vec<&str> = game[1].split(";").collect();
    for round in rounds {
        let mut valid = true;
        let draws: Vec<&str> = round.split(";").map(|el| el.trim()).collect();
        for draw in draws {
            let cube_color_picks: Vec<&str> = draw.split(", ").collect();
            for cube_color_pick in cube_color_picks {
                let pick_component: Vec<&str> = cube_color_pick.split(" ").collect();
                println!("pick: {:?}", pick_component);
                match pick_component[1] {
                    "red" => {
                        if pick_component[0].parse::<i32>().unwrap() > 12 {
                            valid = false;
                        }
                    }
                    "green" => {
                        if pick_component[0].parse::<i32>().unwrap() > 13 {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if pick_component[0].parse::<i32>().unwrap() > 14 {
                            valid = false;
                        }
                    }
                    _ => (),
                }
                if !valid {
                    return 0;
                }
            }
        }
    }

    game_id
}
