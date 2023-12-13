// Turns a string of newline-separated rows into a 2D matrix.
fn parse_patch(input: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        rows.push(line.chars().collect::<Vec<char>>());
    }
    return rows;
}

// Returns the transpose of |a|.
fn transpose(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut b: Vec<Vec<char>> = Vec::new();
    for i in 0..a[0].len() {
        b.push(Vec::new());
        for j in 0..a.len() {
            b[i].push(a[j][i]);
        }
    }
    return b;
}

fn can_hflip(rows: &Vec<Vec<char>>, n: usize, error_correction: bool) -> bool {
    let mut result = true;
    let mut error_corrected_once = false;
    for (i, row) in rows[0..n].iter().enumerate() {
        let iflip = 2 * n - 1 - i;
        if iflip >= rows.len() {
            // Do not check rows that fold to outside the matrix bounds.
            continue;
        }
        for (j, x) in row.iter().enumerate() {
            if rows[iflip][j] != *x {
                // Without error correction, we fail immediately.
                // With error correction, we record any first error and fail on a second error.
                if !error_correction || error_corrected_once {
                    result = false;
                    break;
                }
                error_corrected_once = true;
            }
        }
        if !result {
            break;
        }
    }
    if error_correction {
        return result && error_corrected_once;
    }
    return result;
}

fn compute_num_mirrorings(text: &String, error_correction: bool) -> i64 {
    let patches: Vec<Vec<Vec<char>>> = text.split("\n\n").map(|s| parse_patch(s)).collect();
    let mut hsum = 0;
    let mut vsum = 0;
    for patch in patches {
        // Try all horizontal lines.
        hsum += (1..patch.len())
            .map(|i| match can_hflip(&patch, i as usize, error_correction) {
                true => i,
                false => 0,
            })
            .sum::<usize>() as i64;
        // Try all vertical lines.
        let patch_t = transpose(patch);
        vsum += (1..patch_t.len())
            .map(
                |i| match can_hflip(&patch_t, i as usize, error_correction) {
                    true => i,
                    false => 0,
                },
            )
            .sum::<usize>() as i64;
    }
    return vsum + 100 * hsum;
}

pub fn solve_part_1(text: &String) -> () {
    let answer = compute_num_mirrorings(text, /*error_correction*/ false);

    println!("Number of solutions:    {answer}");
    println!("Expected puzzle answer: 36015");
}

pub fn solve_part_2(text: &String) -> () {
    let answer = compute_num_mirrorings(text, /*error_correction*/ true);

    println!("Number of solutions:    {answer}");
    println!("Expected puzzle answer: 35335");
}
