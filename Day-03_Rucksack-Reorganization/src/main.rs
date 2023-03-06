use std::io;

fn get_priority(c: char) -> u32{
    if c.is_lowercase(){
        return (c as u32 - 'a' as u32) + 1;
    }
    else {
        return (c as u32 - 'A' as u32) + 27;
    }
}

fn main() {
    let mut priority_sum: u32 = 0;
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

    for input in &inputs {
        let first_half = input[0..input.len() / 2].to_string();
        let second_half = input[(input.len() / 2)..input.len()].to_string();

        for c in first_half.chars() {
            if second_half.contains(c) {
                priority_sum += get_priority(c);
                break;
            }
        }
    }
    println!("Part One: {}", priority_sum);

    priority_sum = 0;
    for i in (0..inputs.len()/3){
        let s1 = inputs[3*i].to_string();
        let s2 = inputs[3*i + 1].to_string();
        let s3 = inputs[3*i + 2].to_string();
        for c in s1.chars(){
            if(s2.contains(c) && s3.contains(c)){
                priority_sum += get_priority(c);
                break;
            }
        }
    }
    println!("Part Two: {}", priority_sum);
}
