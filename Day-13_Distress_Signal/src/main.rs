use std::cmp::{min, Ordering};
use std::io;

#[derive(Debug, Clone)]
pub struct ArrayElement
{
    number: i32,
    vector: Vec<ArrayElement>,
    use_number: bool
}

impl ArrayElement
{
    fn new(number: i32, vector: Vec<ArrayElement>, use_number: bool) -> ArrayElement
    {
        return ArrayElement{
            number,
            vector,
            use_number,
        };
    }

    fn number(num: i32) -> ArrayElement
    {
        return ArrayElement{
            number: num,
            vector: Vec::new(),
            use_number: true,
        };
    }

    fn vector(vec: Vec<ArrayElement>) -> ArrayElement
    {
        return ArrayElement{
            number: 0,
            vector: vec,
            use_number: false,
        };
    }

    fn push(&mut self, value: ArrayElement) -> ()
    {
        self.vector.push(value);
    }

    fn to_string(&self) -> String
    {
        let mut result: String = String::new();
        if self.use_number{
            result += &*self.number.to_string();
        }
        else {
            result += "[";
            for (i, elem) in self.vector.iter().enumerate(){
                result += &*elem.to_string();
                if i != self.vector.len() - 1{
                    result += ", ";
                }
            }
            result += "]";
        }

        return result;
    }

    fn at(&mut self, index: usize) -> &mut ArrayElement
    {
        return &mut self.vector[index];
    }
}

macro_rules! arr_num{
    ($a: expr)=>{
        ArrayElement::number($a)
    };
    ()=>{
        ArrayElement::number(0)
    }
}

macro_rules! arr_vec{
    ($a: expr)=>{
        ArrayElement::vector($a)
    };
    ()=>{
        ArrayElement::vector(Vec::new())
    }
}

fn cmp(left: i32, right: i32) -> Ordering{
    return left.cmp(&right);
}

fn compare(left: ArrayElement, right: ArrayElement) -> Ordering{
    if left.use_number && right.use_number{
        return cmp(left.number, right.number);
    }
    if left.use_number && !right.use_number{
        return compare(arr_vec!(vec![arr_num!(left.number)]), right);
    }
    if !left.use_number && right.use_number{
        return compare(left, arr_vec!(vec![arr_num!(right.number)]));
    }

    for i in 0..min(left.vector.len(), right.vector.len()){
        let x = compare(left.vector[i].clone(), right.vector[i].clone());
        if x != Ordering::Equal{
            return x;
        }
    }

    return cmp(left.vector.len() as i32, right.vector.len() as i32);
}

fn parse(mut parsingStr: String) -> ArrayElement
{
    let mut output: ArrayElement = arr_vec!();
    parsingStr = parsingStr[1..parsingStr.len() - 1].parse().unwrap();

    let mut splitted: Vec<String> = Vec::new();
    let mut in_array = 0;

    let mut currentStr: String = String::new();
    for c in parsingStr.chars(){
        if c == '['{
            in_array += 1;
        }
        else if c == ']'{
            in_array -= 1;
        }
        if in_array == 0 && c == ','{
            splitted.push(currentStr.clone());
            currentStr.clear();
        }
        else{
            currentStr.push(c);
        }
    }
    splitted.push(currentStr.clone());

    for i in 0..splitted.len(){
        if splitted[i].starts_with("["){
            output.push(arr_vec!(parse(splitted[i].clone()).vector));
        }
        else if !splitted[i].is_empty(){
            output.push(arr_num!(splitted[i].parse().unwrap()));
        }
    }

    return output;
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    let mut previous_empty = false;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty(){
            if previous_empty{
                break;
            }
            previous_empty = true;
        }
        else {
            previous_empty = false;
        }
        inputs.push(input);
    }

    let mut part_one_sum = 0;
    for i in 0..((inputs.len() + 1)/3) {
        let mut vec_left = parse(inputs[3 * i].clone());
        let mut vec_right = parse(inputs[3 * i + 1].clone());

        if compare(vec_left, vec_right) == Ordering::Less{
            part_one_sum += i + 1;
        }
    }

    println!("Part One: {}", part_one_sum);

    inputs.retain(|x| !(*x).is_empty());
    inputs.push("[[2]]".to_string());
    inputs.push("[[6]]".to_string());

    let mut parsed_inputs = Vec::new();
    for input in inputs{
        parsed_inputs.push(parse(input));
    }

    parsed_inputs.sort_by(|a, b| compare(a.clone(), b.clone()));

    let mut idx1 = 1 + parsed_inputs.iter().position(|r| r.to_string() == "[[2]]".to_string()).unwrap();
    let mut idx2 = 1 + parsed_inputs.iter().position(|r| r.to_string() == "[[6]]".to_string()).unwrap();

    println!("Part Two: {}", idx1 * idx2);
}
