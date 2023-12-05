pub fn solve_part_1(text: &String) -> () {
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
            ranges_lists.push(range);
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
