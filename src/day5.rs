use aoc_runner_derive::aoc;

use itertools::Itertools;
use rayon::prelude::*;
use ropey::Rope;
use hashbrown::HashSet;

fn fix<T: Clone>(mut val: T, fun: impl Fn(&T) -> Option<T>) -> T {
    loop {
        if let Some(new) = fun(&val) {
            val = new;
        } else {
            return val;
        }
    }
}

fn comp_iter<T: Clone>(it: impl IntoIterator<Item = T>) -> impl Iterator<Item = (T, T)> {
    let (a, b) = it.into_iter().tee();
    let b = b.skip(1);

    a.zip(b)
}

fn is_match(a: char, b: char) -> bool {
    (a.is_ascii_uppercase() && b.is_ascii_lowercase() && a == b.to_ascii_uppercase())
        || (b.is_ascii_uppercase() && a.is_ascii_lowercase() && b == a.to_ascii_uppercase())
}

fn step_remove(poly: &Rope) -> Option<Rope> {
    let mut new_poly = poly.clone();

    let (_, did_mutate, _) = comp_iter(poly.chars()).enumerate().fold(
        (false, false, 0),
        |(skip, did_mutate, offset), (idx, (a, b))| {
            if skip {
                return (false, did_mutate, offset);
            }

            if is_match(a, b) {
                let pos = idx - offset;
                new_poly.remove(pos..pos + 2);

                (true, true, offset + 2)
            } else {
                (false, did_mutate, offset)
            }
        },
    );

    if did_mutate {
        Some(new_poly)
    } else {
        None
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    fix(Rope::from_str(input), step_remove).len_chars()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let units: HashSet<_> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    units
        .into_iter()
        .par_bridge()
        .map(|c| {
            let filtered: String = input
                .chars()
                .filter(|ic| ic.to_ascii_lowercase() != c)
                .collect();

            fix(Rope::from_str(&filtered), step_remove).len_chars()
        })
        .min()
        .unwrap()
}
