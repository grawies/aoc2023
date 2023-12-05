use std::cmp;
use std::collections::VecDeque;

// Returns a pair of:
// - a vector of seed tokens, interpreted differently for part 1 and 2.
// - a vector (for each map) of range triples (target range start, source range start, range length).
fn parse_input(text: &String) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
    let mut it = lines.iter();

    let mut ids: Vec<i64> = Vec::new();
    for token in it.next().expect("too short input file")[7..].split(' ') {
        ids.push(token.parse::<i64>().expect("expected integer seed ID"));
    }

    let mut ranges_lists: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut range: Vec<(i64, i64, i64)> = Vec::new();
    loop {
        let line = it.next();
        if line.is_none() {
            ranges_lists.push(range);
            break;
        }
        if line.expect("unreachable?").trim().len() == 0 {
            if range.len() > 0 {
                ranges_lists.push(range);
            }
            range = Vec::new();
            it.next();
            continue;
        }
        let tokens = line
            .expect("unreachable?")
            .split(' ')
            .collect::<Vec<&str>>();
        let mut token_it = tokens.iter();
        let t_begin = token_it
            .next()
            .expect("short input")
            .parse::<i64>()
            .expect("expected integer target range begin");
        let s_begin = token_it
            .next()
            .expect("short input")
            .parse::<i64>()
            .expect("expected integer source range begin");
        let r_len = token_it
            .next()
            .expect("short input")
            .parse::<i64>()
            .expect("expected integer range length");
        range.push((t_begin, s_begin, r_len));
    }
    return (ids, ranges_lists);
}

pub fn solve_part_1(text: &String) -> () {
    let (mut ids, ranges_lists) = parse_input(text);
    for ranges in ranges_lists.iter() {
        for id in &mut ids {
            for (t, s, l) in ranges.iter() {
                if *id >= *s && *id < s + l {
                    *id += t - s;
                    break;
                }
            }
        }
    }
    let smallest_id = ids.iter().min().expect("seed list must not be empty");

    println!("Smallest seed ID:       {}", smallest_id);
    println!("Expected puzzle answer: 382895070");
}

fn are_overlapping((s1, l1): (i64, i64), (s2, l2): (i64, i64)) -> bool {
    if l1 == 0 || l2 == 0 {
        return false;
    }
    // If they do not overlap, then range 1 is either fully below or above range 2.
    let s1_below = s1 + l1 - 1 < s2;
    let s1_above = s1 > s2 + l2 - 1;
    return !s1_below && !s1_above;
}

pub fn solve_part_2(text: &String) -> () {
    let (ids, ranges_lists) = parse_input(text);

    // Parse the seed list into ranges of IDs.
    let mut id_ranges: VecDeque<(i64, i64)> = VecDeque::new();
    for i in (0..ids.len()).step_by(2) {
        id_ranges.push_back((ids[i], ids[i + 1]));
    }

    // Iterate over the queue of ID ranges.
    // If a range in the queue overlaps a range in the mapping, map the overlapping part into the next type.
    // Any non-overlaps are reinserted into the current queue.
    // If nothing matches, the entire range maps as-is into the next type.
    let mut new_ranges: VecDeque<(i64, i64)> = VecDeque::new();
    for map in ranges_lists {
        while let Some((start, length)) = id_ranges.pop_front() {
            let mut mapped = false;
            for (t_start, s_start, r_length) in &map {
                if are_overlapping((start, length), (*s_start, *r_length)) {
                    // Split into overlap and non-overlaps.
                    if start < *s_start {
                        // There is a non-overlap below the target range.
                        id_ranges.push_back((start, s_start - start));
                    }
                    let end = start + length - 1;
                    let s_end = s_start + r_length - 1;
                    if end > s_end {
                        // There is a non-overlap above the target range.
                        id_ranges.push_back((s_end + 1, end - s_end - 1));
                    }
                    let overlap_start = cmp::max(start, *s_start);
                    let overlap_end = cmp::min(end, s_end);
                    let overlap_range_mapped = (
                        t_start - s_start + overlap_start,
                        overlap_end - overlap_start + 1,
                    );
                    new_ranges.push_back(overlap_range_mapped);
                    mapped = true;
                    break;
                }
            }
            if mapped {
                continue;
            }
            // No match - the whole range maps 1:1 to the new type.
            new_ranges.push_back((start, length));
        }
        // Prepare the next loop iteration.
        id_ranges = new_ranges;
        new_ranges = VecDeque::new();
    }

    let smallest_id = id_ranges
        .iter()
        .min_by_key(|(s, _)| s)
        .expect("ID list must not be empty")
        .0;

    println!("Smallest seed ID:       {}", smallest_id);
    println!("Expected puzzle answer: 17729182");
}
