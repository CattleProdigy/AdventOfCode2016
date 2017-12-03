use std::str::FromStr;
use std::env;

fn spiral_for_target_help(target: i32) -> (i32, i32)
{
    assert!(target > 0, "not valid for non-positve vals");
    if target == 1
    {
        return (0, 0);
    }

    let mut spiral_min = 2;
    let mut spiral_max = 9;
    let mut spiral_index = 1;
    let mut x = 1;
    let mut y = 0;
    let mut edge_length = 3;
    let mut spiral_length;
    loop 
    {
        if target >= spiral_min &&
           target <= spiral_max
        {
            break;
        }

        // Successor
        spiral_index += 1;
        edge_length = spiral_index * 2 + 1;
        spiral_length = 4 * edge_length - 4;
        spiral_min = spiral_max + 1;
        spiral_max = spiral_min + spiral_length - 1;
        x += 1;
        y += 1;
    }

    // spiral adjustment
    let mut inter_spiral_dist = target - spiral_min;

    if inter_spiral_dist > 0
    {
        let adjustment = inter_spiral_dist.min(edge_length-2);
        y -= adjustment;
        inter_spiral_dist -= adjustment;
    }

    if inter_spiral_dist > 0
    {
        let adjustment = inter_spiral_dist.min(edge_length-1);
        x -= adjustment;
        inter_spiral_dist -= adjustment;
    }

    if inter_spiral_dist > 0
    {
        let adjustment = inter_spiral_dist.min(edge_length-1);
        y += adjustment;
        inter_spiral_dist -= adjustment;
    }

    if inter_spiral_dist > 0
    {
        let adjustment = inter_spiral_dist.min(edge_length-1);
        x += adjustment;
        inter_spiral_dist -= adjustment;
    }

    assert!(inter_spiral_dist == 0, "We missed something");

    (x, y)
}


fn carry_distance(address: i32) -> i32
{
    let pos = spiral_for_target_help(address);
    pos.0.abs() + pos.1.abs()
}

fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() != 2
    {
        panic!("Provide a number");
    }
    let input_num = i32::from_str(&args[1]).unwrap();
    let carry_dist = carry_distance(input_num);
    println!("Carry Distance {}", carry_dist);
}

#[cfg(test)]
mod test
{
    use carry_distance;

    #[test]
    fn example_1()
    {
        let res = carry_distance(1);
        println!("res: {}", res);
        assert!(res == 0);
    }

    #[test]
    fn example_2()
    {
        let res = carry_distance(12);
        println!("res: {}", res);
        assert!(res == 3);
    }

    #[test]
    fn example_3()
    {
        let res = carry_distance(23);
        println!("res: {}", res);
        assert!(res == 2);
    }

    #[test]
    fn example_4()
    {
        let res = carry_distance(1024);
        println!("res: {}", res);
        assert!(res == 31);
    }
}

