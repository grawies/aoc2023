use std::collections::LinkedList;

type HashMap = Vec<LinkedList<MapEntry>>;

fn hash(s: &str) -> usize {
    return s
        .chars()
        .map(|c| c as u8 as usize)
        .fold::<usize, _>(0, |h, x| ((h + x) * 17) % 256);
}

pub fn solve_part_1(text: &String) -> () {
    let instructions: Vec<String> = text.split(",").map(|s| s.to_string()).collect();
    let hash_sum: usize = instructions.iter().map(|s| hash(&s)).sum();

    println!("Hash of all instructions: {hash_sum}");
    println!("Expected puzzle answer:   522547");
}

struct MapEntry {
    // TODO: Replace this with &str and learn the lifetime stuff.
    label: String,
    value: i64,
}

fn insert(label: &str, value: i64, m: &mut HashMap) {
    let list = &mut m[hash(label)];
    for e in list.iter_mut() {
        if e.label == label {
            e.value = value;
            return;
        }
    }
    list.push_back(MapEntry {
        label: label.to_string(),
        value: value,
    });
}

fn remove(label: &str, m: &mut HashMap) -> () {
    let list = &mut m[hash(label)];
    match list.iter().position(|e| e.label == label) {
        Some(i) => {
            let mut tail = list.split_off(i);
            tail.pop_front();
            list.append(&mut tail);
        }
        None => {}
    }
}

pub fn solve_part_2(text: &String) -> () {
    let instructions: Vec<String> = text.split(",").map(|s| s.to_string()).collect();

    // Initialize hashmap.
    let mut hashmap: HashMap = Vec::new();
    for _ in 0..256 {
        hashmap.push(LinkedList::new());
    }

    // Execute all of the instructions.
    for instruction in instructions {
        if instruction.contains('-') {
            remove(&instruction[0..instruction.len() - 1], &mut hashmap);
        } else {
            let (label, value_str) = instruction.split_at(instruction.find('=').unwrap() + 1);
            insert(
                &label[..label.len() - 1],
                value_str.parse::<i64>().unwrap(),
                &mut hashmap,
            );
        }
    }

    // Compute the answer sum from the resulting hashmap.
    let mut content_sum: i64 = 0;
    for (box_index, list) in hashmap.iter().enumerate() {
        for (slot_index, entry) in list.iter().enumerate() {
            content_sum += (box_index as i64 + 1) * (slot_index as i64 + 1) * entry.value;
        }
    }

    println!("Summary of map contents: {content_sum}");
    println!("Expected puzzle answer:  229271");
}
