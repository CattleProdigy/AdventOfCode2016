use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::env;
use std::cmp;

fn checksum(input_str: &String) -> i32
{
    let mut line_checksums: Vec<i32> = Vec::new();
    for line in input_str.lines()
    {
        let mut min_val: i32 = i32::max_value();
        let mut max_val: i32 = i32::min_value();
        for s in line.split(char::is_whitespace)
        {
            let curr_int = i32::from_str(s).unwrap();
            min_val = curr_int.min(min_val);
            max_val = curr_int.max(max_val);
        }
        line_checksums.push(max_val - min_val);
    }

    let sum = line_checksums.iter()
                            .fold(0, |acc, &x| acc + x);

    sum
}

fn row_divisors(input_str: &String) -> i32
{
    let mut line_divisors: Vec<i32> = Vec::new();
    for line in input_str.lines()
    {
        let line_digits: Vec<i32> = line.split(char::is_whitespace)
                                        .map(&i32::from_str)
                                        .map(&std::result::Result::unwrap)
                                        .collect();
        
        let a = 
            (0..line_digits.len()-1).flat_map(|i| line_digits[i+1..].into_iter()
                                                                    .map(|j| (line_digits[i], j)).collect::<Vec<_>>())
            .filter(|&(a, b)| cmp::max(a, *b) % cmp::min(a, *b)== 0)
            .map(|(a, b)| cmp::max(a, *b) / cmp::min(a, *b))
            .fold(0, |acc, x| acc + x);
        line_divisors.push(a)
    }
    
    let sum = line_divisors.iter()
                           .fold(0, |acc, &x| acc + x);

    sum
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

    let cs = checksum(&file_str);
    println!("Checksum: {}", cs);
    let ds = row_divisors(&file_str);
    println!("Divisorsum: {}", ds);
}

#[cfg(test)]
mod test
{
    use checksum;
    use row_divisors;

    #[test]
    fn provided_example()
    {
        let example: String = "5 1 9 5\n\
                               7 5 3\n\
                               2 4 6 8".into();
        assert!(checksum(&example) == 18);
    }

    #[test]
    fn provided_example2()
    {
        let example: String = "5 9 2 8\n\
                               9 4 7 3\n\
                               3 8 6 5".into();
        assert!(row_divisors(&example) == 9);
    }
}
