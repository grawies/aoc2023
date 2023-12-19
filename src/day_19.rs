use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64, line_ending, one_of},
    combinator::all_consuming,
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

use std::cmp;
use std::collections::HashMap;

type Block = ((i64, i64), (i64, i64), (i64, i64), (i64, i64));

fn volume((x, m, a, s): Block) -> i64 {
    return (1 + x.1 - x.0) * (1 + m.1 - m.0) * (1 + a.1 - a.0) * (1 + s.1 - s.0);
}

pub fn solve_part_1(text: &String) -> () {
    let (workflows, parts) = parse_input(text);
    let initial_blocks = parts
        .iter()
        .map(|(x, m, a, s)| ((*x, *x), (*m, *m), (*a, *a), (*s, *s)))
        .collect::<Vec<Block>>();
    let accepted_rating_sum = get_accepted_blocks(workflows, initial_blocks)
        .iter()
        .fold(0, |sum, block| {
            sum + block.0 .0 + block.1 .0 + block.2 .0 + block.3 .0
        });

    println!("Sum of accepted part ratings: {accepted_rating_sum}");
    println!("Expected puzzle answer:       374873");
}

pub fn solve_part_2(text: &String) -> () {
    let (workflows, _) = parse_input(text);
    let initial_blocks = vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000))];
    let accepted_volume = get_accepted_blocks(workflows, initial_blocks)
        .iter()
        .fold(0, |sum, block| sum + volume(*block));

    println!("Number of valid rating combos: {accepted_volume}");
    println!("Expected puzzle answer:        122112157518711");
}

fn parse_input(
    input: &str,
) -> (
    HashMap<&str, (Vec<(char, char, i64, &str)>, &str)>,
    Vec<(i64, i64, i64, i64)>,
) {
    let (workflows_text, parts_text) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, (Vec<(char, char, i64, &str)>, &str)> =
        match all_consuming(separated_list1(
            line_ending,
            tuple((
                alpha1::<_, Error<_>>,
                delimited(
                    tag("{"),
                    tuple((
                        many0(terminated(
                            tuple((
                                one_of("xmas"),
                                one_of("<>"),
                                i64,
                                preceded(tag(":"), alpha1),
                            )),
                            tag(","),
                        )),
                        alpha1,
                    )),
                    tag("}"),
                ),
            )),
        ))(workflows_text)
        {
            Err(e) => {
                panic!("bad input: {}", e);
            }
            Ok((_, result)) => HashMap::from_iter(result),
        };

    let parts = match all_consuming(separated_list1(
        line_ending,
        delimited(
            tag::<_, _, Error<_>>("{"),
            tuple((
                preceded(tag("x="), i64),
                preceded(tag(",m="), i64),
                preceded(tag(",a="), i64),
                preceded(tag(",s="), i64),
            )),
            tag("}"),
        ),
    ))(parts_text)
    {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, result)) => result,
    };

    return (workflows, parts);
}

fn get_accepted_blocks(
    workflows: HashMap<&str, (Vec<(char, char, i64, &str)>, &str)>,
    initial_blocks: Vec<Block>,
) -> Vec<Block> {
    // Approach:
    // Start with the initial blocks.
    // Each rule will split the block along some axis, creating two (potentially empty) new blocks.
    // Keep applying rules and splitting blocks, sending them to different workflows.
    // Discard any empty blocks encountered.
    // When all blocks are accepted or rejected, stop and return the accepted blocks.

    // Tests a rule against a block and returns the passing/failing ranges as a pair (pass,fail).
    fn apply_rule(b: Block, field: char, op: char, limit: i64) -> (Option<Block>, Option<Block>) {
        let (offset_lo, offset_hi, lo_pass) = match op {
            '<' => (-1, 0, true),
            '>' => (0, 1, false),
            _ => panic!("bad operator symbol {op}"),
        };
        let (b_lo, b_hi) = match field {
            'x' => (
                ((b.0 .0, cmp::min(b.0 .1, limit + offset_lo)), b.1, b.2, b.3),
                ((cmp::max(b.0 .0, limit + offset_hi), b.0 .1), b.1, b.2, b.3),
            ),
            'm' => (
                (b.0, (b.1 .0, cmp::min(b.1 .1, limit + offset_lo)), b.2, b.3),
                (b.0, (cmp::max(b.1 .0, limit + offset_hi), b.1 .1), b.2, b.3),
            ),
            'a' => (
                (b.0, b.1, (b.2 .0, cmp::min(b.2 .1, limit + offset_lo)), b.3),
                (b.0, b.1, (cmp::max(b.2 .0, limit + offset_hi), b.2 .1), b.3),
            ),
            's' => (
                (b.0, b.1, b.2, (b.3 .0, cmp::min(b.3 .1, limit + offset_lo))),
                (b.0, b.1, b.2, (cmp::max(b.3 .0, limit + offset_hi), b.3 .1)),
            ),
            _ => panic!("bad part field {field}"),
        };
        let volume_or_none = |(x, m, a, s): Block| {
            if x.1 >= x.0 && m.1 >= m.0 && a.1 >= a.0 && s.1 >= s.0 {
                return Some((x, m, a, s));
            }
            return None;
        };
        if lo_pass {
            return (volume_or_none(b_lo), volume_or_none(b_hi));
        } else {
            return (volume_or_none(b_hi), volume_or_none(b_lo));
        }
    }

    let mut stack: Vec<(&str, Block)> = Vec::from_iter(initial_blocks.iter().map(|b| ("in", *b)));
    let mut accepted_blocks: Vec<Block> = Vec::new();
    while let Some((label, block)) = stack.pop() {
        if label == "R" {
            continue;
        }
        if label == "A" {
            accepted_blocks.push(block);
            continue;
        }
        let workflow = workflows.get(label).unwrap();
        // Apply each rule, splitting the block of values into sub-blocks that either pass or fail each rule.
        // Some blocks may be empty - we clear those when they are popped from the stack.
        let mut remaining_block = Some(block);
        for rule in &workflow.0 {
            if remaining_block.is_none() {
                break;
            }
            let (b_pass, b_fail) = apply_rule(remaining_block.unwrap(), rule.0, rule.1, rule.2);
            if b_pass.is_some() {
                stack.push((rule.3, b_pass.unwrap()));
            }
            remaining_block = b_fail;
        }
        if remaining_block.is_some() {
            stack.push((workflow.1, remaining_block.unwrap()));
        }
    }
    return accepted_blocks;
}
