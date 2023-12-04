use std::collections::HashMap;
use std::collections::HashSet;

// Returns tuple of card number and the number of winning numbers on the card.
fn score_card(card_string: &String) -> (i32, i32) {
    let i;
    match card_string.find(":") {
        None => {
            panic!("unparseable line: {}", card_string);
        }
        Some(x) => {
            i = x;
        }
    }
    let card_number = card_string[5..i]
        .trim()
        .parse::<i32>()
        .expect("expected an integer");
    let num_string = card_string[i + 1..].to_string();
    let mut token_iter = num_string.trim().split(' ').filter(|s| s.len() > 0);
    let mut winning_numbers: HashSet<i32> = HashSet::new();
    while let Some(token) = token_iter.next() {
        if token == "|" {
            break;
        }
        let number = token.parse::<i32>().expect("expected an integer");
        winning_numbers.insert(number);
    }
    let mut num_winning = 0;
    while let Some(token) = token_iter.next() {
        let number = token.parse::<i32>().expect("expected an integer");
        if winning_numbers.contains(&number) {
            num_winning += 1;
        }
    }
    return (card_number, num_winning);
}

pub fn solve_part_1(text: &String) -> () {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut total_points = 0;
    for line in lines {
        let (_, num_winning) = score_card(&line);
        if num_winning > 0 {
            total_points += 1 << (num_winning - 1);
        }
    }
    println!("Card points sum:        {}", total_points);
    println!("Expected puzzle answer: 17803");
}

pub fn solve_part_2(text: &String) -> () {
    let mut lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut num_cards_generated_from_card: HashMap<i32, i32> = HashMap::new();
    // By iterating in reverse, all copies generated by a card have already been processed.
    lines.reverse();
    let mut num_cards = 0;
    for line in lines {
        num_cards += 1;
        let (card_number, num_winning) = score_card(&line);
        if num_winning == 0 {
            num_cards_generated_from_card.insert(card_number, 0);
            continue;
        }
        let mut num_generated = 0;
        for i in card_number + 1..card_number + num_winning + 1 {
            num_generated += 1;
            num_generated += *num_cards_generated_from_card.entry(i).or_default();
        }
        num_cards_generated_from_card.insert(card_number, num_generated);
        num_cards += num_generated;
    }
    println!("Number of cards generated {}", num_cards);
    println!("Expected puzzle answer:   5554894");
}
