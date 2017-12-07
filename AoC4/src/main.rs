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

fn valid_passphrase2(input: &str) -> bool
{
    let mut unique_strs = HashSet::new();
    for str in input.split(|c: char| c.is_whitespace())
    {
        let mut chars: Vec<char> = str.chars().collect();
        chars.sort();

        let sorted_str: String = chars.into_iter().collect();
        if !unique_strs.insert(sorted_str)
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

    let num_vp2 = input_str.split("\n")
                          .filter(|&s| !s.is_empty())
                          .map(&valid_passphrase2)
                          .filter(|&vp| vp)
                          .count();
    println!("Number of valid passphrases 2: {}", num_vp2);
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

    use valid_passphrase2;
    #[test]
    fn example4()
    {
        let ex_str = "abcde fghij";
        assert!(valid_passphrase2(&ex_str));
    }

    #[test]
    fn example5()
    {
        let ex_str = "abcde xyz ecdab";
        assert!(!valid_passphrase2(&ex_str));
    }
    #[test]
    fn example6()
    {
        let ex_str = "a ab abc abd abf abj";
        assert!(valid_passphrase2(&ex_str));
    }
    #[test]
    fn example7()
    {
        let ex_str = "iiii oiii ooii oooi oooo";
        assert!(valid_passphrase2(&ex_str));
    }
    #[test]
    fn example8()
    {
        let ex_str = "oiii ioii iioi iiio";
        assert!(!valid_passphrase2(&ex_str));
    }

}
