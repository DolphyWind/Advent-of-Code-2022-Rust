use std::io;

fn check_cycle(cycle: i32) -> bool{
    return (cycle - 20) % 40 == 0;
}

fn convert_to_index(number: i32, width: i32) -> (i32, i32){
    let y = number / width;
    let x = number % width;
    return (x, y);
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

    let mut X: i32 = 1;
    let mut current_cycle: i32 = 1;
    let mut total_sum: i32 = 0;
    let mut screen: [[bool; 40]; 6] = [[false; 40]; 6];

    for input in &inputs{
        if check_cycle(current_cycle){
            total_sum += X * current_cycle;
        }

        let crt_pos = convert_to_index(current_cycle - 1, screen[0].len() as i32);

        if X - 1 == crt_pos.0{
            screen[crt_pos.1 as usize][(X - 1) as usize] = true;
        }
        else if X == crt_pos.0 {
            screen[crt_pos.1 as usize][X as usize] = true;
        }
        else if X + 1 == crt_pos.0{
            screen[crt_pos.1 as usize][(X + 1) as usize] = true;
        }

        if input == "noop"{
            current_cycle += 1;
        }
        else {
            let increment_str = input.split(" ").collect::<Vec<&str>>()[1].to_string();
            let increment: i32 = increment_str.parse().unwrap();

            if check_cycle(current_cycle + 1){
                total_sum += X * (current_cycle + 1);
            }

            let next_crt = convert_to_index(current_cycle, screen[0].len() as i32);

            if X - 1 == next_crt.0{
                screen[next_crt.1 as usize][(X - 1) as usize] = true;
            }
            else if X == next_crt.0 {
                screen[next_crt.1 as usize][X as usize] = true;
            }
            else if X + 1 == next_crt.0{
                screen[next_crt.1 as usize][(X + 1) as usize] = true;
            }

            current_cycle += 2;
            X += increment;
        }
    }
    println!("Part One: {}", total_sum);

    println!("Part Two:");
    for y in 0..screen.len(){
        for x in 0..screen[0].len(){
            if screen[y][x]{
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }

}
