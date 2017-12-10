use std::collections::HashSet;
use std::io::{self, Read};

fn balance_banks(input: &Vec<i32>) -> Vec<i32>
{
    let mut max_ind = 0;
    let mut max = <i32>::min_value(); 
    for i in 0..input.len()
    {
        if input[i] > max
        {
            max_ind = i;
            max = input[i];
        }
    }

    let mut input_copy = input.clone();
    input_copy[max_ind] = 0;
    for _ in 0..max
    {
        max_ind = (max_ind + 1) % input_copy.len();
        input_copy[max_ind] += 1;
    }
    input_copy
}

fn balance_til_repeat(input: &Vec<i32>) -> (i32, Vec<i32>)
{
    let mut steps = 0;
    let mut banks = HashSet::new();
    banks.insert(input.clone());
    let mut curr_bank = input.clone();
    loop
    {
        steps += 1;
        curr_bank = balance_banks(&curr_bank);
        if !banks.insert(curr_bank.clone())
        {
            break;
        }
    }
    (steps, curr_bank)
}

fn main()
{

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
               .expect("couldn't read from stdin");

    let input_banks: Vec<i32> = buffer.split(|x| char::is_whitespace(x))
                                      .filter(|x| x.len() > 0)
                                      .map(|x| str::parse::<i32>(x).unwrap())
                                      .collect();

    let (steps, next_bank) = balance_til_repeat(&input_banks);
    println!("Steps: {}", steps);
    let (steps2, _) = balance_til_repeat(&next_bank);
    println!("Second Steps: {}", steps2);
}

#[cfg(test)]
mod test
{
    use balance_til_repeat;

    #[test]
    fn example1()
    {
        let example = vec![0, 2, 7, 0];
        let (steps, _) = balance_til_repeat(&example);
        assert!(steps == 5);
    }

    #[test]
    fn example2()
    {
        let example = vec![0, 2, 7, 0];
        let (_, bank) = balance_til_repeat(&example);
        let (steps2, _) = balance_til_repeat(&bank);
        assert!(steps2 == 4);
    }
}


