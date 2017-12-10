use std::io::{self, Read};
use std::collections::HashMap;

pub struct Node
{
    id: String,
    weight: i32,
    children: Vec<String>
}

impl Node
{
    pub fn new() -> Node
    {
        let id = String::new();
        let weight: i32 = 0;
        let children = Vec::new();
        Node {id, weight, children}
    }
}

fn build_tree(input_str: &str) -> HashMap<&str, Node>
{
    let mut tree = HashMap::new();
    for l in input_str.lines()
    {
        let tokens: Vec<&str> = l.split(|c| char::is_whitespace(c))
                                 .collect();
        let mut new_node = Node::new();

        // Id
        let id = tokens[0];
        new_node.id = id.into();

        // Weight
        let weight_str: String = tokens[1].to_string();
        let slice = &weight_str[1..weight_str.len()-1];
        let weight = str::parse::<i32>(&slice).unwrap();
        new_node.weight = weight;

        // Children
        if tokens.len() > 2
        {
            for i in 3..tokens.len()-1
            {
                let child_str: String = tokens[i].into();
                let child_str_trimmed = &child_str[..child_str.len()-1];
                new_node.children.push(child_str_trimmed.to_string());
            }
            let child_str: String = tokens[tokens.len()-1].into();
            new_node.children.push(child_str);
        }
        tree.insert(id, new_node);
    }

    tree
}

fn max_depth_node_id_help(tree: &HashMap<&str, Node>, curr_node: &str, depth: i32) -> i32
{
    if tree[curr_node].children.len() == 0
    {
        return depth;
    }

    let children = &tree[curr_node].children;
    children.iter().map(|c| max_depth_node_id_help(tree, c, depth+1))
                   .max().unwrap()
}

fn max_depth_node_id(tree: &HashMap<&str, Node>) -> String
{
    let (k, _) = tree.iter().map(|(&k, _)| (k, max_depth_node_id_help(&tree, k, 0)))
                            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                            .unwrap();
    k.to_string()
}

fn find_imbalance(tree: &HashMap<&str, Node>, curr_node: &str) -> i32
{
    if tree[curr_node].children.len() == 0
    {
        return tree[curr_node].weight;
    }

    let children = &tree[curr_node].children;
    let child_weights: Vec<i32> = children.iter()
                                          .map(|c| find_imbalance(&tree, c))
                                          .collect();
                        
    let val: i32 = child_weights[0];
    let mut mismatch = false;
    for c in child_weights.iter()
    {
        if c != &val
        {
            mismatch = true;
        }
    }

    if mismatch
    {
        println!("Mismatch: {}", tree[curr_node].id);
        for (c2, id) in child_weights.iter().zip(children.iter())
        {
            let id_copy: &str = &id.clone();
            let weight = tree[id_copy].weight;
            println!("\t{}: ({}) : {}", id, weight, c2);
        }
    }
    
    child_weights.iter()
                 .fold(0, |acc, &x| acc + x)
                 + tree[curr_node].weight
}

fn main()
{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
               .expect("Couldn't read from stdin");

    let tree = build_tree(&buffer);

    let bottom =  max_depth_node_id(&tree);
    println!("Bottom: {}", bottom);

    let imbalance = find_imbalance(&tree, &bottom);
    println!("Imbalance: {}", imbalance);
}

#[cfg(test)]
mod test
{
    #[test]
    fn example1()
    {
        assert!(false);
    }

}
