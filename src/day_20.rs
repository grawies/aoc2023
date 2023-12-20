use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{all_consuming, opt},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, tuple},
};

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve_part_1(text: &String) -> () {
    let (low_pulses, high_pulses, _) = press_button_collect_data(text);
    let answer = low_pulses * high_pulses;

    println!("Product of num high/low pulses: {answer}");
    println!("Expected puzzle answer:         821985143");
}

pub fn solve_part_2(text: &String) -> () {
    let (_, _, key_press_periods) = press_button_collect_data(text);
    let answer = key_press_periods
        .iter()
        .fold(1, |prod, period| prod * period);

    println!("Button presses until rx turned on: {answer}");
    println!("Expected puzzle answer:            240853834793347");
}

// Presses the button repeatedly and returns a tuple of:
// - the number of low pulses observed within the first 1000 button presses,
// - the number of high pulses observed within the first 1000 button presses, and
// - the (relatively prime) number of presses required to observe a low pulse from four key nodes for part two.
fn press_button_collect_data(text: &str) -> (i64, i64, Vec<i64>) {
    let module_map: HashMap<&str, (Option<char>, Vec<&str>)> = parse_modules(text);
    let terminal_node = (None, vec![]);
    let mut flipflop_state: HashMap<&str, bool> = HashMap::new();
    let mut conj_state: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    for (name, (type_symbol, destinations)) in &module_map {
        if *type_symbol == Some('%') {
            flipflop_state.insert(name, true);
        }
        for dest in destinations {
            if module_map.get(dest).unwrap_or(&terminal_node).0 == Some('&') {
                conj_state.entry(dest).or_default().insert(name, true);
            }
        }
    }

    // Press the button until we know:
    // - the number of low and high pulses after 1000 presses, and
    // - the number of presses needed to get a low pulse out of each key nodes for part two.
    let mut button_press_count: i64 = 0;
    let mut low_pulse_counter: i64 = 0;
    let mut high_pulse_counter: i64 = 0;
    let mut key_press_periods: HashMap<&str, i64> = HashMap::new();
    let key_nodes = vec!["th", "nt", "ff", "zs"];
    while button_press_count < 1000 || key_press_periods.len() < key_nodes.len() {
        // Simulate the pulse propagation with a queue, enqueueing any module that receives a new pulse.
        button_press_count += 1;
        if button_press_count <= 1000 {
            low_pulse_counter += 1;
        }
        let mut queue: VecDeque<(&str, &str, bool)> =
            VecDeque::from([("button", "broadcaster", true)]);
        while let Some((input_name, name, low_pulse)) = queue.pop_front() {
            let (type_symbol, destinations) = module_map.get(name).unwrap_or(&terminal_node);

            // Update state and compute module output.
            let new_pulse: Option<bool>;
            match type_symbol {
                Some('%') => {
                    // Flip-flip.
                    if low_pulse {
                        flipflop_state.entry(name).and_modify(|b| {
                            *b = !*b;
                        });
                        new_pulse = Some(*flipflop_state.get(name).unwrap());
                    } else {
                        new_pulse = None;
                    }
                }
                Some('&') => {
                    // Conjugation.
                    let inputs = conj_state.get_mut(name).unwrap();
                    inputs.insert(input_name, low_pulse);
                    let is_any_input_low = inputs.values().any(|b| *b);
                    new_pulse = Some(!is_any_input_low);
                }
                None => {
                    // Broadcaster or debug output.
                    new_pulse = Some(low_pulse);
                }
                _ => {
                    panic!("unexpected type symbol: {}", type_symbol.unwrap());
                }
            }
            match new_pulse {
                Some(pulse) => {
                    if button_press_count <= 1000 {
                        if pulse {
                            low_pulse_counter += destinations.len() as i64;
                        } else {
                            high_pulse_counter += destinations.len() as i64;
                        }
                    }
                    if pulse && key_nodes.contains(&name) {
                        key_press_periods.entry(name).or_insert(button_press_count);
                    }
                    // Propagate to destination modules.
                    for dest in destinations {
                        queue.push_back((name, dest, pulse));
                    }
                }
                _ => {}
            }
        }
    }

    return (
        low_pulse_counter,
        high_pulse_counter,
        key_press_periods.into_values().collect(),
    );
}

// Returns map from module name to module type and list of destination module names.
fn parse_modules(input: &str) -> HashMap<&str, (Option<char>, Vec<&str>)> {
    // Parse the data into a reasonable data type.
    let module_data: Vec<(Option<char>, &str, Vec<&str>)> = all_consuming(separated_list1(
        line_ending,
        tuple((
            // module type (broadcaster is None)
            opt(one_of("%&")),
            // module name
            alpha1::<_, Error<_>>,
            // destination modules
            preceded(tag(" -> "), separated_list1(tag(", "), alpha1)),
        )),
    ))(input)
    .unwrap()
    .1;

    // Turn it into a hashmap before returning.
    return HashMap::from_iter(
        module_data
            .iter()
            .map(|(type_sym, name, dest_vec)| (*name, (*type_sym, dest_vec.clone()))),
    );
}
