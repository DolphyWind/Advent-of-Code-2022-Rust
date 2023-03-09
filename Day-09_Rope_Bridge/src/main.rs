use std::collections::HashSet;
use std::io;

fn is_adjacent(tailPos: (i32, i32), headPos: (i32, i32)) -> bool{
    let mut delta_x: i32 = tailPos.0 - headPos.0;
    let mut delta_y: i32 = tailPos.1 - headPos.1;
    delta_x = delta_x.abs();
    delta_y = delta_y.abs();

    return delta_x <= 1 && delta_y <= 1;
}

fn should_move_horizontal_or_vertical(tailPos: (i32, i32), headPos: (i32, i32)) -> bool{
    let mut delta_x: i32 = tailPos.0 - headPos.0;
    let mut delta_y: i32 = tailPos.1 - headPos.1;
    delta_x = delta_x.abs();
    delta_y = delta_y.abs();

    return (delta_x == 0 && delta_y != 0) || (delta_y == 0 && delta_x != 0);
}

fn follow(mut tail_coords: (i32, i32), mut head_coords: (i32, i32)) -> (i32, i32){
    if !is_adjacent(tail_coords, head_coords){
        if should_move_horizontal_or_vertical(tail_coords, head_coords){
            let mut delta_x: i32 = head_coords.0 - tail_coords.0;
            let mut delta_y: i32 = head_coords.1 - tail_coords.1;
            tail_coords.0 += delta_x / 2;
            tail_coords.1 += delta_y / 2;
        }
        else {

            'outer: for i in 0..2{
                for j in 0..2{
                    let d_x = (i * 2) - 1;
                    let d_y = (j * 2) - 1;
                    let mut temp_tail = tail_coords;
                    temp_tail.0 += d_x;
                    temp_tail.1 += d_y;

                    if is_adjacent(temp_tail, head_coords){
                        tail_coords = temp_tail;
                        break 'outer;
                    }
                }
            }
        }
    }
    return tail_coords;
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty(){
            break;
        }
        inputs.push(input);
    }

    let mut first_tail_set: HashSet<(i32, i32)> = HashSet::new();
    let mut last_tail_set: HashSet<(i32, i32)> = HashSet::new();
    let mut head_coords = (0, 0);
    let mut tails: [(i32, i32); 9] = [(0, 0); 9];


    for input in &inputs{
        let splitted = input.split(" ").collect::<Vec<&str>>();
        let direction = splitted[0];
        let steps = splitted[1].parse::<i32>().unwrap();

        for _i in 0..steps{
            if direction == "D"{
                head_coords.1 -= 1;
            }
            else if direction == "U"{
                head_coords.1 += 1;
            }
            else if direction == "L"{
                head_coords.0 -= 1;
            }
            else if direction == "R"{
                head_coords.0 += 1;
            }

            tails[0] = follow(tails[0], head_coords);
            for i in 1..tails.len(){
                tails[i] = follow(tails[i], tails[i-1]);
            }

            first_tail_set.insert(tails[0]);
            last_tail_set.insert(tails[8]);
        }
    }

    println!("Part One: {}", first_tail_set.len());
    println!("Part Two: {}", last_tail_set.len());

}
