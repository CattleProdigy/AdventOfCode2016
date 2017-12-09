use std::env;
use std::fs::File;
use std::io::prelude::*;

fn vector_successor(input: &mut Vec<i32>, curr_index: i32) -> i32
{
    let ind = curr_index as usize;
    let new_index = input[ind] + (ind as i32);
    input[ind] = input[ind] + 1;
    new_index 
}

fn vector_successor2(input: &mut Vec<i32>, curr_index: i32) -> i32
{
    let ind = curr_index as usize;
    let offset = input[ind];
    let new_index = offset + (ind as i32);
    if offset >= 3
    {
        input[ind] = input[ind] - 1;
    }
    else
    {
        input[ind] = input[ind] + 1;
    }
    new_index 
}

fn steps_to_exit(input: &Vec<i32>) -> i32
{
    let mut input_copy = input.clone();
    let mut curr_index = 0;
    let mut num_steps = 0;
    loop
    {
        curr_index = vector_successor(&mut input_copy, curr_index); 
        num_steps = num_steps + 1;

        if curr_index >= input_copy.len() as i32 ||
           curr_index < 0
        {
            break;
        }
    }
    num_steps
}

fn steps_to_exit2(input: &Vec<i32>) -> i32
{
    let mut input_copy = input.clone();
    let mut curr_index = 0;
    let mut num_steps = 0;
    loop
    {
        curr_index = vector_successor2(&mut input_copy, curr_index); 
        num_steps = num_steps + 1;

        if curr_index >= input_copy.len() as i32 ||
           curr_index < 0
        {
            break;
        }
    }
    num_steps
}

fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() != 2
    {
        panic!("Provide a path to input");
    }

    let input_path = &args[1];
    let mut file = File::open(input_path).expect("Unable to open file");

    let mut file_str = String::new();
    file.read_to_string(&mut file_str).expect("Error reading file");

    let input_vec: Vec<i32> = file_str.lines()
                                      .map(|x| x.parse::<i32>().unwrap())
                                      .collect();

    println!("steps: {}", steps_to_exit(&input_vec));
    println!("steps2: {}", steps_to_exit2(&input_vec));
}

#[cfg(test)]
mod test
{
    use steps_to_exit;
    use steps_to_exit2;

    #[test]
    fn example1()
    {
        let example = vec![0, 3, 0, 1, -3];
        assert!(steps_to_exit(&example) == 5);
    }

    #[test]
    fn example2()
    {
        let example = vec![0, 3, 0, 1, -3];
        assert!(steps_to_exit2(&example) == 10);
    }
}
