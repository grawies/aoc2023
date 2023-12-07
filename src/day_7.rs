use nom::{
    character::complete::{alphanumeric1, space1, u64},
    combinator::all_consuming,
    error::Error,
    sequence::separated_pair,
};

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn compute_type_value(hand: &str) -> u64 {
    let values_set: HashSet<char> = HashSet::from_iter(hand.chars());
    let mut value_counter: HashMap<char, u64> = HashMap::new();
    for c in hand.chars() {
        value_counter
            .entry(c)
            .and_modify(|e| *e = *e + 1)
            .or_insert(1);
    }
    let mut type_value = 0;
    if values_set.len() == 1 {
        // Five of a kind.
        type_value = 7;
    } else if values_set.len() == 2 {
        let mut four_of_a_kind = false;
        for c in value_counter.keys() {
            if *value_counter.get(c).expect("value missing from counters") == 4 {
                four_of_a_kind = true;
                // Four of a kind.
                type_value = 6;
                break;
            }
        }
        if !four_of_a_kind {
            // Full house.
            type_value = 5;
        }
    } else if value_counter.values().position(|x| *x == 3).is_some() {
        // Three of a kind.
        type_value = 4;
    } else if values_set.len() == 3 {
        // Two pairs.
        type_value = 3;
    } else if values_set.len() == 4 {
        // One pair.
        type_value = 2;
    } else {
        // High card.
        type_value = 1;
    }
    return type_value;
}

fn compute_card_key(hand: &str) -> u64 {
    assert!(hand.len() == 5);
    let val_map = vec![
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let values = hand
        .chars()
        .map(|c| val_map.iter().position(|k| k == &c))
        .map(|o| o.expect("char missing from card value map") as u64)
        .collect::<Vec<u64>>();
    let mut key: u64 = 0;
    for v in values {
        key = 0x10 * key + v;
    }
    return key;
}

// We assign each hand a unique key in the range 0x0 - 0x7fffff:
// - the most significant digit is the kind,
// - the next five hex digits are the value of each card.
// This allows them to be compared by key.
fn compute_key(hand: &str) -> u64 {
    let type_value = compute_type_value(hand);
    let card_key = compute_card_key(hand);
    return type_value * 0x100000 + card_key;
}

fn compute_max_key_with_joker(hand: &str) -> u64 {
    let mut type_value = 0;
    if hand == "JJJJJ" {
        // Short circuit the all-joker input, to simplify the substitution code.
        type_value = 7;
    } else {
        // Try all combinations of joker substitutions, keeping the one that maximizes type value.

        // Adding a new card value will always yield a lower or equal score to adding a copy of a pre-existing card.
        // Hence we only need to try substitutions of cards already in the hand.
        // There is always at least one, since we handle the JJJJJ case above.
        let non_joker_cards: HashSet<char> = HashSet::from_iter(hand.chars().filter(|c| *c != 'J'));
        let mut queue: VecDeque<String> = VecDeque::from([String::from(hand)]);
        while let Some(h) = queue.pop_front() {
            if h.contains('J') {
                for sub in non_joker_cards.iter() {
                    queue.push_back(h.replacen("J", &String::from(*sub), 1));
                }
                continue;
            }
            // No jokers - ready to evaluate.
            type_value = cmp::max(type_value, compute_type_value(&h));
        }
    }
    let card_key = compute_card_key(&hand.replace("J", "1"));
    return type_value * 0x100000 + card_key;
}

fn parse_row(input: &str) -> (&str, u64) {
    match all_consuming(separated_pair(alphanumeric1::<_, Error<_>>, space1, u64))(input) {
        Err(e) => {
            panic!("bad input: {}", e);
        }
        Ok((_, output)) => {
            return output;
        }
    }
}

pub fn solve_part_1(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut hands: Vec<(&str, u64)> = lines.iter().map(|s| parse_row(s)).collect();
    hands.sort_by_key(|(h, _)| compute_key(h));

    let mut total_winnings = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        let rank = i + 1;
        total_winnings += (rank as u64) * *bid;
    }
    println!("Total winnings:         {}", total_winnings);
    println!("Expected puzzle answer: 250370104");
}

pub fn solve_part_2(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut hands: Vec<(&str, u64)> = lines.iter().map(|s| parse_row(s)).collect();
    hands.sort_by_key(|(h, _)| compute_max_key_with_joker(h));

    let mut total_winnings = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        let rank = i + 1;
        total_winnings += (rank as u64) * *bid;
    }
    println!("Total winnings:         {}", total_winnings);
    println!("Expected puzzle answer: 251735672");
}
