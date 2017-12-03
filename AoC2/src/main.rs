use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::env;

fn checksum(input_str: String) -> i32
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

    let cs = checksum(file_str);
    println!("Checksum: {}", cs);
}

#[cfg(test)]
mod test
{
    use checksum;

    #[test]
    fn provided_example()
    {
        let example: String = "5 1 9 5\n\
                               7 5 3\n\
                               2 4 6 8".into();
        assert!(checksum(example) == 18);
    }
}
