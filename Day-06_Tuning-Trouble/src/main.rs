use std::io;

fn is_all_distinct(s: String) -> bool{
    let mut seen = String::new();
    for c in s.chars(){
        if seen.contains(c){
            return false;
        }
        seen.push(c);
    }

    return true;
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("");
    input = input[0..input.len() - 1].to_string();
    let mut part_one_solved = false;
    let mut part_two_solved = false;

    for i in 0..input.len() - 13{
        let substr1 = (&input[i..i+4]).to_string();
        let substr2 = (&input[i..i+14]).to_string();

        if !part_one_solved && is_all_distinct(substr1.clone()){
            println!("Part One: {}", i + substr1.len());
            part_one_solved = true;
        }
        if !part_two_solved && is_all_distinct(substr2.clone()){
            println!("Part Two: {}", i + substr2.len());
            part_two_solved = true;
        }
    }
}
