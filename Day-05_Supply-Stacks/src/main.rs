use std::io;
use std::process::exit;


fn cargo_move(n: u32, from: &mut String, to: &mut String){
    for _i in 0..n{
        (*to).push((*from).pop().unwrap());
    }
}

fn cargo_move_second(n: u32, from: &mut String, to: &mut String){
    let mut temp_str: String = String::new();

    for _i in 0..n{
        temp_str.push((*from).pop().unwrap());
    }

    for _i in 0..n{
        (*to).push(temp_str.pop().unwrap());
    }
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

    let mut cargo_vec: Vec<String> = Vec::new();
    cargo_vec.push(String::new()); // Won't be used
    cargo_vec.push("DTWFJSHN".to_string());
    cargo_vec.push("HRPQTNBG".to_string());
    cargo_vec.push("LQV".to_string());
    cargo_vec.push("NBSWRQ".to_string());
    cargo_vec.push("NDFTVMB".to_string());
    cargo_vec.push("MDBVHTR".to_string());
    cargo_vec.push("DBQJ".to_string());
    cargo_vec.push("DNJVRZHQ".to_string());
    cargo_vec.push("BNHMS".to_string());

    for input in &inputs{
        let splitted: Vec<&str> = input.split(" ").collect();
        let n: u32 = splitted[1].parse::<u32>().unwrap();
        let from_index: usize = splitted[3].parse::<usize>().unwrap();
        let to_index: usize = splitted[5].parse::<usize>().unwrap();

        let mut from = std::mem::take(&mut cargo_vec[from_index]);
        let mut to = std::mem::take(&mut cargo_vec[to_index]);

        cargo_move(n, &mut from, &mut to);

        cargo_vec[from_index] = from;
        cargo_vec[to_index] = to;
    }

    let mut part_one_str:String = String::new();
    for cargo in &cargo_vec{
        if cargo.is_empty() {
            continue;
        }
        part_one_str.push(cargo.chars().nth(cargo.len()-1).unwrap());
    }
    println!("Part One: {}", part_one_str);

    let mut cargo_vec_second: Vec<String> = Vec::new();
    cargo_vec_second.push(String::new()); // Won't be used
    cargo_vec_second.push("DTWFJSHN".to_string());
    cargo_vec_second.push("HRPQTNBG".to_string());
    cargo_vec_second.push("LQV".to_string());
    cargo_vec_second.push("NBSWRQ".to_string());
    cargo_vec_second.push("NDFTVMB".to_string());
    cargo_vec_second.push("MDBVHTR".to_string());
    cargo_vec_second.push("DBQJ".to_string());
    cargo_vec_second.push("DNJVRZHQ".to_string());
    cargo_vec_second.push("BNHMS".to_string());

    for input in &inputs{
        let splitted: Vec<&str> = input.split(" ").collect();
        let n: u32 = splitted[1].parse::<u32>().unwrap();
        let from_index: usize = splitted[3].parse::<usize>().unwrap();
        let to_index: usize = splitted[5].parse::<usize>().unwrap();

        let mut from = std::mem::take(&mut cargo_vec_second[from_index]);
        let mut to = std::mem::take(&mut cargo_vec_second[to_index]);

        cargo_move_second(n, &mut from, &mut to);

        cargo_vec_second[from_index] = from;
        cargo_vec_second[to_index] = to;
    }

    let mut part_two_str:String = String::new();
    for cargo in cargo_vec_second{
        if cargo.is_empty() {
            continue;
        }
        part_two_str.push(cargo.chars().nth(cargo.len()-1).unwrap());
    }
    println!("Part Two: {}", part_two_str);
}
