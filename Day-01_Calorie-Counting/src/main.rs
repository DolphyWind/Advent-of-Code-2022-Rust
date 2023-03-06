use std::io;

// https://adventofcode.com/2022/day/1

fn main() {

    let mut caloriesVector: Vec<i32> = Vec::<i32>::new();
    let mut currentCalories: i32 = 0;
    loop{

        let mut caloriesStr = String::new();
        io::stdin().read_line(&mut caloriesStr).expect("failed to readline");
        caloriesStr = caloriesStr[0..caloriesStr.len() - 1].to_string();
        
        if caloriesStr.is_empty(){
            if currentCalories == 0{
                break;
            }
            caloriesVector.push(currentCalories);
            currentCalories = 0;
        }
        else {
            let calories: i32 = caloriesStr.parse::<i32>().unwrap();
            currentCalories += calories;
        }
    }
    
    caloriesVector.sort_by(|a, b| b.cmp(a));

    println!("Part One: {:?}", caloriesVector[0]);
    println!("Part Two: {:?}", caloriesVector[0] + caloriesVector[1] + caloriesVector[2]);
    
}
