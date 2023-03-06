use std::io;

fn does_fully_contain(a: (u32, u32), b: (u32, u32)) -> bool{
    return b.0 >= a.0 && b.1 <= a.1;
}

fn does_overlap(a: (u32, u32), b: (u32, u32)) -> bool{
    return (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.0 && b.1 <= a.1);
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    let mut fully_contains_count: u32 = 0;
    let mut overlap_count: u32 = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty(){
            break;
        }
        inputs.push(input);
    }

    for input in &inputs{
        let mut splitted: Vec<&str> = input.split(",").collect();
        let mut first_elf_str: Vec<&str> = splitted[0].split("-").collect();
        let mut second_elf_str: Vec<&str> = splitted[1].split("-").collect();

        let first_elf = (first_elf_str[0].parse::<u32>().unwrap(), first_elf_str[1].parse::<u32>().unwrap());
        let second_elf = (second_elf_str[0].parse::<u32>().unwrap(), second_elf_str[1].parse::<u32>().unwrap());
        if does_fully_contain(first_elf, second_elf) || does_fully_contain(second_elf, first_elf){
            fully_contains_count += 1;
        }

        if does_overlap(first_elf, second_elf) || does_overlap(second_elf, first_elf){
            overlap_count += 1;
        }
    }
    println!("Part One: {}", fully_contains_count);
    println!("Part Two: {}", overlap_count);
}
