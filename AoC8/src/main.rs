use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Opcode
{
    Decr= -1,
    Incr= 1
}

impl Opcode
{
    fn from_str(s: &str) -> Result<Self, &str>
    {
        match s
        {
            "inc" => Ok(Opcode::Incr), 
            "dec" => Ok(Opcode::Decr), 
            _ => Err("Can't parse this guy")
        }
    }
}

#[derive(PartialEq)]
enum CondOpcode
{
    Gt,
    Lt,
    Eq,
    Neq,
    Lte,
    Gte
}

impl CondOpcode
{
    fn from_str(s: &str) -> Result<Self, &str>
    {
        match s
        {
            ">" => Ok(CondOpcode::Gt), 
            "<" => Ok(CondOpcode::Lt), 
            "==" => Ok(CondOpcode::Eq), 
            "!=" => Ok(CondOpcode::Neq), 
            "<=" => Ok(CondOpcode::Lte), 
            ">=" => Ok(CondOpcode::Gte), 
            _ => Err("Can't parse this guy")
        }
    }
    
    fn func(&self) -> (fn(i32, i32) -> bool) 
    {
        match self
        {
            &CondOpcode::Gt => |x:i32, y:i32| x > y,
            &CondOpcode::Lt => |x:i32, y:i32| x < y,
            &CondOpcode::Eq => |x:i32, y:i32| x == y,
            &CondOpcode::Neq => |x:i32, y:i32| x != y,
            &CondOpcode::Lte => |x:i32, y:i32| x <= y,
            &CondOpcode::Gte => |x:i32, y:i32| x >= y,
        }
    }
}

pub struct Inst
{
    reg: String,
    opcode: Opcode,
    op_arg: i32,
    cond_opcode: CondOpcode,
    cond_op_arg_l: String,
    cond_op_arg_r: i32
}
impl Inst
{
    pub fn new() -> Inst
    {
        Inst {reg: String::new(),
              opcode: Opcode::Incr,
              op_arg: 0,
              cond_opcode: CondOpcode::Lt,
              cond_op_arg_l: String::new(),
              cond_op_arg_r: 0}
    }
}

fn run_process(instrs: &Vec<Inst>) -> (i32, i32)
{
    let mut regs: HashMap<&str, i32> = HashMap::new();
    let mut maxmax = i32::min_value();
    for i in instrs
    {
        let left_opcode_val = *regs.entry(&i.cond_op_arg_l)
                                   .or_insert(0);
        if i.cond_opcode.func()(left_opcode_val, i.cond_op_arg_r)
        {
            let opcode_multi = i.opcode.clone() as i32;
            *regs.entry(&i.reg)
                 .or_insert(0) += opcode_multi * i.op_arg;
        }

        let curr_max = *regs.values()
                            .max() 
                            .unwrap();
        maxmax = maxmax.max(curr_max);
    }

    let reg_max = *regs.values()
                       .max() 
                       .unwrap();

    (reg_max, maxmax)
}

fn parse_instructions(input: &str) -> Vec<Inst>
{
    let mut instrs: Vec<Inst> = Vec::new();
    for l in input.lines()
    {
        let mut new_instr: Inst = Inst::new();

        let tokens: Vec<&str> = l.split(|c| char::is_whitespace(c))
                                 .collect();

        new_instr.reg = tokens[0].into();
        new_instr.opcode = Opcode::from_str(&tokens[1]).unwrap();
        new_instr.op_arg = tokens[2].parse::<i32>().unwrap();
        new_instr.cond_op_arg_l = tokens[4].into();
        new_instr.cond_opcode = CondOpcode::from_str(&tokens[5]).unwrap();
        new_instr.cond_op_arg_r = tokens[6].parse::<i32>().unwrap();

        instrs.push(new_instr);
    }
    instrs
}

fn main()
{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
               .expect("Couldn't read from stdin");

    let instrs = parse_instructions(&buffer);
    let (reg_max, process_max) = run_process(&instrs);
    println!("Max Reg Val: {}, {}", reg_max, process_max);
}

#[cfg(test)]
mod test
{
    use CondOpcode;
    use run_process;
    use parse_instructions;

    #[test]
    fn test_condition_opcode()
    {
        let lt: CondOpcode = CondOpcode::Lt;
        let gt: CondOpcode = CondOpcode::Gt;
        let eq: CondOpcode = CondOpcode::Eq;
        let neq: CondOpcode = CondOpcode::Neq;
        let lte: CondOpcode = CondOpcode::Lte;
        let gte: CondOpcode = CondOpcode::Gte;
        let lts = CondOpcode::from_str("<").unwrap();
        let gts = CondOpcode::from_str(">").unwrap();
        let eqs = CondOpcode::from_str("==").unwrap();
        let neqs = CondOpcode::from_str("!=").unwrap();
        let ltes = CondOpcode::from_str("<=").unwrap();
        let gtes = CondOpcode::from_str(">=").unwrap();
        assert!(lts == lt);
        assert!(gts == gt);
        assert!(eqs == eq);
        assert!(neqs == neq);
        assert!(ltes == lte);
        assert!(gtes == gte);
        let x = -1;
        let y = 1;
        assert!(lt.func()(x, y));
        assert!(!gt.func()(x, y));
        assert!(!eq.func()(x, y));
        assert!(neq.func()(x, y));
        assert!(lte.func()(x, y));
        assert!(!gte.func()(x, y));
    }

    #[test]
    fn test_input()
    {
        let in_str = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

        let instrs= parse_instructions(in_str);

        assert!(run_process(&instrs).0 == 1);
    }
}
