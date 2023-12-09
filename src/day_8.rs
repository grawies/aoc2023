use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::all_consuming,
    error::Error,
    sequence::{separated_pair, terminated},
};

use std::collections::HashMap;

// Parse lines of format "x = (y, z)" for alphanumeric IDs x, y, z.
fn parse_node(input: &str) -> (&str, &str, &str) {
    match all_consuming(separated_pair(
        alphanumeric1::<_, Error<_>>,
        tag(" = ("),
        terminated(
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    ))(input)
    {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, (a, (b, c)))) => {
            return (a, b, c);
        }
    }
}

pub fn solve_part_1(text: &String) -> () {
    // Parse input.
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let instruction = &lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines[2..] {
        let (source, left, right) = parse_node(line);
        map.insert(source, (left, right));
    }

    // Walk the graph from "AAA" until we hit "ZZZ".
    let mut current = "AAA";
    let mut num_steps = 0;
    while current != "ZZZ" {
        let choice = map.get(current).unwrap();
        current = match instruction[num_steps % instruction.len()] {
            'L' => choice.0,
            'R' => choice.1,
            _ => panic!("unrecognized instruction"),
        };
        num_steps += 1;
    }

    println!("Number of steps to reach ZZZ: {num_steps}");
    println!("Expected puzzle answer:       14681");
}

pub fn solve_part_2(text: &String) -> () {
    // Parse input.
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let instruction = &lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut currents: Vec<&str> = Vec::new();
    for line in &lines[2..] {
        let (source, left, right) = parse_node(line);
        map.insert(source, (left, right));
        if source.chars().last().unwrap() == 'A' {
            currents.push(source);
        }
    }

    // For each start node, walk the node until we find a cycle.
    // Note all potential terminal nodes we pass along the way:
    // - into the cycle, and
    // - within the cycle.
    let instr_len = instruction.len() as i64;
    type Node<'a> = (&'a str, i64);
    let mut cycle_time: Vec<i64> = Vec::new();
    for current in &currents {
        let mut visited: HashMap<Node, i64> = HashMap::new();
        let mut terminals: Vec<i64> = Vec::new();
        let mut steps = 0;
        let mut node: Node = (current, 0);
        while !visited.contains_key(&node) {
            visited.insert(node, steps);
            let choice = map.get(node.0).unwrap();
            let c = match instruction[(steps % instr_len) as usize] {
                'L' => choice.0,
                'R' => choice.1,
                _ => panic!("unrecognized instruction"),
            };
            steps += 1;
            node = (c, steps % instr_len);
            if node.0.chars().last().unwrap() == 'Z' {
                terminals.push(steps);
            }
        }
        let enter_t = *visited.get(&node).unwrap();
        let cycle_t = steps - enter_t;
        cycle_time.push(cycle_t);
    }

    // Notes for full input:
    // - each cycle is a multiple of 277
    // - each path only hits one terminal
    // - it takes exactly one cycle length to reach the terminal
    // - all multiples are mutually prime
    // - my multiples are 61, 59, 79, 47, 53, 73, 277
    // in conclusion, the answer is 277 * 61*59*79*47*53*73*277 = 14321394058031
    let mut num_steps: i64 = instr_len;
    for c in cycle_time {
        num_steps *= c / instr_len;
    }

    println!("Number of steps for all paths to terminal: {}", num_steps);
    println!("Expected puzzle answer:                    14321394058031");
}
