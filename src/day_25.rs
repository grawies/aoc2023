use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
};

use rand::Rng;

use std::collections::HashMap;
use std::collections::HashSet;

// Karger's Algorithm.
// https://en.wikipedia.org/wiki/Karger%27s_algorithm
// Returns a tuple of the edge cut size and the size of each resulting partition.
// Has about 0.5% chance of finding the true minimum cut on my problem input.
fn try_min_cut(
    neighbors_orig: Vec<Vec<usize>>,
    edges_orig: HashSet<(usize, usize)>,
) -> (usize, usize, usize) {
    let mut neighbors = neighbors_orig.clone();
    let mut edges = edges_orig.iter().cloned().collect::<Vec<(usize, usize)>>();
    let mut contraction_size: Vec<usize> = Vec::from_iter((0..neighbors.len()).map(|_| 1));
    // Keep track of which vertices are contracted into which.
    // If edge (i,j) is contracted into i, then repr[j] = i.
    let mut repr: Vec<usize> = Vec::from_iter(0..neighbors.len());

    let mut num_removed_last = 0;
    let mut last_i = 0;
    let mut last_j = 0;
    let mut rng = rand::thread_rng();
    while edges.len() > 0 {
        let random_index = rng.gen_range(0..edges.len());
        let (mut i, mut j) = edges.swap_remove(random_index);
        while repr[i] != i {
            i = repr[i];
        }
        while repr[j] != j {
            j = repr[j];
        }
        if i == j {
            // Edge has already been contracted into a self-loop, ignore.
            continue;
        }
        // Contract this edge, merging j into i.
        let nbrs_j = neighbors[j].clone();
        // Update i and all j-neighbors to point to each other.
        for k in &nbrs_j {
            for l in neighbors[*k].iter_mut() {
                if *l == j {
                    *l = j;
                }
            }
            neighbors[i].push(*k);
        }
        // Update all edges to refer to i instead of j, deleting any self-loops that arise.
        let mut to_remove: Vec<usize> = Vec::new();
        for (idx, (u, v)) in edges.iter_mut().enumerate() {
            if *u == j {
                *u = i;
            }
            if *v == j {
                *v = i;
            }
            if *u == *v {
                to_remove.push(idx);
            }
        }
        to_remove.reverse();
        for idx in &to_remove {
            edges.swap_remove(*idx);
        }

        repr[j] = i;
        if edges.len() == 0 {
            num_removed_last = 1 + to_remove.len();
            last_i = i;
            last_j = j;
            break;
        }
        contraction_size[i] += contraction_size[j];
    }

    return (
        num_removed_last,
        contraction_size[last_i],
        contraction_size[last_j],
    );
}

pub fn solve_part_1(text: &String) -> () {
    let connections = parse_rows(text);
    let mut names: Vec<&str> = Vec::new();
    let mut ids: HashMap<&str, usize> = HashMap::new();
    let mut neighbors: Vec<Vec<usize>> = Vec::new();
    let mut edges: HashSet<(usize, usize)> = HashSet::new();

    fn get_or_create_id<'a>(
        s: &'a str,
        is: &mut HashMap<&'a str, usize>,
        ns: &mut Vec<&'a str>,
        nbrs: &mut Vec<Vec<usize>>,
    ) -> usize {
        if is.contains_key(s) {
            return *is.get(s).unwrap();
        }
        let i = ns.len();
        ns.push(s);
        is.insert(s, i);
        nbrs.push(Vec::new());
        return i;
    }

    for (name, adj) in connections {
        let i = get_or_create_id(name, &mut ids, &mut names, &mut neighbors);
        for nbr_name in adj {
            let j = get_or_create_id(nbr_name, &mut ids, &mut names, &mut neighbors);
            edges.insert((i, j));
            neighbors[i].push(j);
        }
    }

    let answer: usize;
    let mut num_attempts = 0;
    loop {
        num_attempts += 1;
        let (cut_size, a_size, b_size) = try_min_cut(neighbors.clone(), edges.clone());
        if cut_size == 3 {
            answer = a_size * b_size;
            break;
        }
    }

    println!("Product of partition sizes: {answer} (after {num_attempts} attempts)");
    println!("Expected puzzle answer:     582692");
}

fn parse_rows(input: &str) -> Vec<(&str, Vec<&str>)> {
    return all_consuming(separated_list1(
        line_ending::<_, Error<_>>,
        separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
    ))(input)
    .unwrap()
    .1;
}
