use aoc_runner_derive::{aoc, aoc_generator, Runner};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|n| n.parse().unwrap()) // collecting Some would be slower afaik
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.into_iter().sum()
}

#[aoc(day1, part2, StdLibHashSet)]
pub fn part2_stdlib(input: &[i32]) -> i32 {
    use std::collections::HashSet;

    // assume not that many uniques
    let mut set = HashSet::new();

    input
        .into_iter()
        .cycle()
        .try_fold(0, |freq, n| {
            if !set.insert(freq) {
                Err(freq)
            } else {
                Ok(freq + n)
            }
        })
        .unwrap_err()
}

#[aoc(day1, part2, IntHashSet)]
pub fn part2_inthash(input: &[i32]) -> i32 {
    use int_hash::IntBuildHasher;
    use std::collections::HashSet;

    // assume not that many uniques
    let mut set = HashSet::<_, IntBuildHasher>::default();

    input
        .into_iter()
        .cycle()
        .try_fold(0, |freq, n| {
            if !set.insert(freq) {
                Err(freq)
            } else {
                Ok(freq + n)
            }
        })
        .unwrap_err()
}
