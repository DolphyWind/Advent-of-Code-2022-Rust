use std::io;

/// Solution of part 2
/// For part 1 divide result by 3 in Monkey::operate function and set max_rounds to 20.

static mut SUPER_MOD: u128 = 1;

pub struct Monkey{
    pub starting_items: Vec<u128>,
    pub divisibility_test_num: u128,
    pub operation_str: String,
    pub true_numbers: Vec<u128>,
    pub false_numbers: Vec<u128>,
    pub total_passes: u128,
    pub true_index: usize,
    pub false_index: usize,
}

impl Monkey{

    pub fn operate(&mut self){

        self.total_passes += self.starting_items.len() as u128;
        // println!("{}", self.total_passes);

        for i in 0..self.starting_items.len(){
            let num= self.starting_items[i];
            let mut first_number = 0;
            let mut second_number = 0;

            let splitted = self.operation_str.split(" ").collect::<Vec<&str>>();
            if splitted[0] == "old"{
                first_number = num;
            }
            else {
                first_number = splitted[0].parse().unwrap();
            }
            if splitted[2] == "old"{
                second_number = num;
            }
            else {
                second_number = splitted[2].parse().unwrap();
            }

            let mut result = match splitted[1] {
                "+" => first_number + second_number,
                "-" => first_number - second_number,
                "*" => first_number * second_number,
                "/" => first_number / second_number,
                _ => 0,
            };

            unsafe {result %= SUPER_MOD;};

            if self.test(result){
                self.true_numbers.push(result);
            }
            else {
                self.false_numbers.push(result);
            }

        }

        self.starting_items.clear();
    }

    pub fn test(&self, number: u128) -> bool{
        return number % self.divisibility_test_num == 0;
    }

    pub fn start_new_round(&mut self){
        self.true_numbers.clear();
        self.false_numbers.clear();
    }
}

fn main() {
    let mut inputs: Vec<Vec<String>> = Vec::new();
    let mut current_input: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty(){
            if current_input.is_empty(){
                break;
            }
            inputs.push(current_input.clone());
            current_input.clear();
        }
        else {
            current_input.push(input);
        }
    }

    let max_rounds = 10000;

    let mut monkeys: Vec<Monkey> = Vec::new();
    for input in &inputs{
        let starting_items_vec= &(input[1].split(" ").collect::<Vec<&str>>())[4..];
        let mut start_items: Vec<u128> = Vec::new();
        for s in starting_items_vec{
            let mut tmp = s.to_string();
            if tmp.ends_with(","){
                tmp = tmp[0..tmp.len() - 1].to_string();
            }
            start_items.push(tmp.parse::<u128>().unwrap());
        }

        let operation_str: String = input[2].split(" ").collect::<Vec<&str>>()[5..].join(" ").to_string();
        let divisibility_number: u128 = input[3].split(" ").collect::<Vec<&str>>().last().unwrap().parse().unwrap();
        let true_monkey_index: usize = input[4].split(" ").collect::<Vec<&str>>().last().unwrap().parse().unwrap();
        let false_monkey_index: usize = input[5].split(" ").collect::<Vec<&str>>().last().unwrap().parse().unwrap();

        monkeys.push(Monkey{
            starting_items: start_items.clone(),
            divisibility_test_num: divisibility_number,
            operation_str: operation_str.clone(),
            true_numbers: Vec::new(),
            false_numbers: Vec::new(),
            total_passes: 0,
            true_index: true_monkey_index,
            false_index: false_monkey_index,
        });

        unsafe { SUPER_MOD *= divisibility_number; }
    }

    unsafe { println!("{}", SUPER_MOD); }

    for _i in 0..max_rounds{
        for i in 0..monkeys.len(){
            monkeys[i].start_new_round();
            monkeys[i].operate();

            let mut true_nums = monkeys[i].true_numbers.clone();
            let mut false_nums = monkeys[i].false_numbers.clone();
            let mut true_idx = monkeys[i].true_index;
            let mut false_idx = monkeys[i].false_index;

            monkeys[true_idx].starting_items.append(&mut true_nums);
            monkeys[false_idx].starting_items.append(&mut false_nums);
        }
    }

    let mut total_passes_vec: Vec<u128> = Vec::new();
    for m in &monkeys{
        total_passes_vec.push(m.total_passes);
    }

    total_passes_vec.sort_by(|a, b| b.cmp(a));
    println!("Part two: {}", total_passes_vec[0] * total_passes_vec[1]);
}
