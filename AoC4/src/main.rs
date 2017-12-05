use std::collections::HashSet;
use std::io;
use std::io::Read;

fn valid_passphrase(input: &str) -> bool
{
    let mut unique_strs = HashSet::new();
    for str in input.split(|c: char| c.is_whitespace())
    {
        if !unique_strs.insert(str)
        {
            return false;     
        }
    }
    true
}

fn main()
{
    let mut buffer = String::new(); 
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let input_str: &str = buffer.as_ref();
    let num_vp = input_str.split("\n")
                          .filter(|&s| !s.is_empty())
                          .map(&valid_passphrase)
                          .filter(|&vp| vp)
                          .count();
    println!("Number of valid passphrases: {}", num_vp);
}

#[cfg(test)]
mod test
{
    use valid_passphrase;

    #[test]
    fn example1()
    {
        let ex_str = "aa bb cc dd ee";
        assert!(valid_passphrase(&ex_str));
    }

    #[test]
    fn example2()
    {
        let ex_str = "aa bb cc dd aa";
        assert!(!valid_passphrase(&ex_str));
    }

    #[test]
    fn example3()
    {
        let ex_str = "aa bb cc dd aaa";
        assert!(valid_passphrase(&ex_str));
    }

}
