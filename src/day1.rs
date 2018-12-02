use aoc_runner_derive::{aoc_generator, aoc, Runner};

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

    let mut freq = 0;

    for i in input.into_iter().cycle() {
        if !set.insert(freq) {
            return freq;
        }

        freq += i;
    }

    unreachable!();
}

#[aoc(day1, part2, IntHashSet)]
pub fn part2_inthash(input: &[i32]) -> i32 {
    use std::collections::HashSet;
    use int_hash::IntBuildHasher;

    // assume not that many uniques
    let mut set = HashSet::<_, IntBuildHasher>::default();

    let mut freq = 0;

    for i in input.into_iter().cycle() {
        if !set.insert(freq) {
            return freq;
        }

        freq += i;
    }

    unreachable!();
}
