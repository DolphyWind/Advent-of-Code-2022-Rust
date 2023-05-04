use std::io;

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

    const width: usize = 1000;
    const height: usize = 250;
    let mut cave: [[char; width]; height] = [['.'; width]; height];
    const cave_left: usize = 0;
    let mut max_y: usize = 0;

    for line in inputs{
        let split_str = line.split(" -> ");

        let mut prevX: usize = usize::MAX;
        let mut prevY: usize = usize::MAX;

        for coords in split_str{
            let seperate_coords: Vec<&str> = coords.split(",").collect();
            let mut x: usize = seperate_coords[0].to_string().parse::<usize>().unwrap() - cave_left;
            let mut y: usize = seperate_coords[1].to_string().parse::<usize>().unwrap();
            if y > max_y{
                max_y = y;
            }
            if prevX == usize::MAX && prevY == usize::MAX{
                prevX = x;
                prevY = y;
                cave[y][x] = '#';
            }
            else {
                if x == prevX{
                    let small_y: usize = if y <= prevY {y} else {prevY};
                    let big_y: usize = if small_y == y {prevY} else {y};
                    for i in (small_y)..(big_y + 1){
                        cave[i][x] = '#';
                    }
                }
                else {
                    let small_x = if x <= prevX {x} else {prevX};
                    let big_x = if small_x == x {prevX} else {x};
                    for i in (small_x)..(big_x + 1){
                        cave[y][i] = '#';
                    }
                }
                prevX = x;
                prevY = y;
            }
        }
    }
    for i in 0..width{
        cave[max_y + 2][i] = '#';
    }

    // Part One
    /*
    let mut part_one_answer = 0;
    const sand_start_x: usize = 500 - cave_left;
    const sand_start_y: usize = 0;
    let mut loop_finished: bool = false;

    loop {
        let mut sand_x: usize = sand_start_x;
        let mut sand_y: usize = sand_start_y;
        loop {
            if cave[sand_y + 1][sand_x] == '.' {
                sand_y += 1;
            } else if cave[sand_y + 1][sand_x - 1] == '.' {
                sand_x -= 1;
                sand_y += 1;
            } else if cave[sand_y + 1][sand_x + 1] == '.' {
                sand_x += 1;
                sand_y += 1;
            } else {
                cave[sand_y][sand_x] = 'o';
                break;
            }

            if sand_y >= height - 1{
                loop_finished = true;
                break;
            }
        }
        if loop_finished{
            break;
        }
        part_one_answer += 1;
    }

    println!("Part One: {}", part_one_answer);
    */

    // part two
    let mut part_two_answer = 0;
    const sand_start_x: usize = 500 - cave_left;
    const sand_start_y: usize = 0;
    loop {
        let mut sand_x: usize = sand_start_x;
        let mut sand_y: usize = sand_start_y;
        if cave[sand_y][sand_x] == 'o'{
            break;
        }

        loop {
            if cave[sand_y + 1][sand_x] == '.' {
                sand_y += 1;
            } else if cave[sand_y + 1][sand_x - 1] == '.' {
                sand_x -= 1;
                sand_y += 1;
            } else if cave[sand_y + 1][sand_x + 1] == '.' {
                sand_x += 1;
                sand_y += 1;
            } else {
                cave[sand_y][sand_x] = 'o';
                break;
            }

            if sand_y >= height - 1{
                cave[sand_y][sand_x] = 'o';
                break;
            }
        }
        part_two_answer += 1;
    }

    println!("Part Two: {}", part_two_answer);
}
