use aoc_runner_derive::{aoc, aoc_generator, Runner};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|n| n.parse().unwrap()) // collecting Some would be slower afaik
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2, StdLibHashSet)]
pub fn part2_stdlib(input: &[i32]) -> i32 {
    // assume not that many uniques
    let mut set = HashSet::new();

    input
        .iter()
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

    // assume not that many uniques
    let mut set = HashSet::<_, IntBuildHasher>::default();

    input
        .iter()
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


// math from https://www.reddit.com/r/adventofcode/comments/a20646/2018_day_1_solutions/eaukxu5/

#[aoc(day1, part2, ActuallyGood)]
pub fn part2_actuallygood(input: &[i32]) -> i32 {
    use itertools::{Itertools, MinMaxResult};
    use std::collections::HashMap;

    let mut freq = 0;
    let mut frequencies = Vec::new();

    for n in input {
        frequencies.push(freq);
        freq += n;
    }

    let shift = freq;

    if shift == 0 {
        return 0;
    }

    let mut groups: HashMap<_, Vec<_>> = HashMap::new();

    for (i, f) in frequencies.iter().enumerate() {
        groups.entry(f % shift).or_default().push((i, f));
    }

    let max_diff = match frequencies.iter().minmax() {
        MinMaxResult::NoElements | MinMaxResult::OneElement(..) => return 0,
        MinMaxResult::MinMax(min, max) => max - min,
    };

    for val in groups.values_mut() {
        val.sort_unstable_by_key(|(_, f)| *f);
    }

    let (_, _, min_freq) =
        groups
            .values()
            .fold((frequencies.len(), max_diff, 0), |min, val| {
                let it = val.iter().cloned();
                let mut it2 = val.iter().cloned();
                it2.next();

                it.zip(it2).fold(
                    min,
                    |(min_idx, min_diff, min_freq), ((a_i, a_f), (b_i, b_f))| {
                        let diff = b_f - a_f;

                        let (idx, &freq) = if shift > 0 { (a_i, b_f) } else { (b_i, a_f) };

                        if diff < min_diff || (diff == min_diff && idx < min_idx) {
                            (idx, diff, freq)
                        } else {
                            (min_idx, min_diff, min_freq)
                        }
                    },
                )
            });

    min_freq
}
