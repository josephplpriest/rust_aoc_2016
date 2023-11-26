use std::fs;

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

#[derive(Debug)]
struct Direction {
    turn_direction: String,
    distance: u8,
}
impl Direction {
    fn new(instruction: &Vec<char>) -> Direction {
        let string_num: String = instruction[1..instruction.len()].iter().collect();
        Direction {
            turn_direction: String::from(instruction[0]),
            distance: string_num.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}


fn main() {
    let contents = read_file_string(&String::from("./input.txt"));
    let file_content_string = contents.unwrap();

    let contents_unpacked = file_content_string.split(", ");

    let new_contents =  contents_unpacked.map(|x| String::from(x));

    let mut current_position: Point = Point {x:0, y:0};

    let mut current_direction = "N";

    let compass = vec!["N", "E", "S", "W"];

    let mut visited: Vec<Point> = Vec::new();

    for inst in new_contents {
        let instruction: Vec<char> = inst.chars().collect();
        let d = Direction::new(&instruction);

        let new_idx: i32;

        if visited.iter().any(|&x| 
        (x.x == current_position.x && x.y == current_position.y)
        ) {
            println!("{:?}", current_position);
            panic!("We found it!")
        } else {
        visited.push(current_position.clone());
        println!("{:?}", visited)
        };


        if d.turn_direction == "L" {
            let temp_index = compass.iter().position(|&x|x == current_direction).unwrap() as i32 - 1;
            // hacky rust modulus for negative numbers
            new_idx = ((temp_index % 4) + 4) % 4
        } else {
            new_idx = (compass.iter().position(|&x|x == current_direction).unwrap() as i32 + 1) % 4;
        }
        println!("{}, {:?}, {new_idx}", current_direction, d);
        current_direction = compass[new_idx as usize];
        
        current_position = match current_direction {
            "N" => { 
                Point {x: current_position.x, y: current_position.y + d.distance as i16}
            },
            "S" => { 
                Point {x: current_position.x, y: current_position.y - d.distance as i16}
                },
            "E" => { 
                Point {x: current_position.x  + d.distance as i16, y: current_position.y}
            },
            "W" => { 
                Point {x: current_position.x  - d.distance as i16, y: current_position.y}
            },
            &_ => panic!("That's not a cardinal direction!!")
        };
        
    }

}