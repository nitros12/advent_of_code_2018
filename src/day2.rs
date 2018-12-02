use aoc_runner_derive::{aoc, Runner};

fn count_n(s: &str) -> (bool, bool) {
    s.chars().fold((false, false), |(two, three), c| {
        let count = s.chars().filter(|&ic| c == ic).count();

        match count {
            2 => (true, three),
            3 => (two, true),
            _ => (two, three),
        }
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let (two, three) = input.lines().fold((0, 0), |(two, three), line| {
        let (has_two, has_three) = count_n(line);
        (two + has_two as u32, three + has_three as u32)
    });

    two * three
}

fn count_diff(s1: &str, s2: &str) -> bool {
    match s1.chars().zip(s2.chars()).try_fold(0, |acc, (a, b)| {
        if (acc + (a != b) as u8) > 1 {
            None
        } else {
            Some(acc + (a != b) as u8)
        }
    }) {
        Some(1) => true,
        _ => false,
    }
}

fn filter_diff(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let ids: Vec<_> = input.lines().collect();

    for (idx, id1) in ids.iter().enumerate() {
        for id2 in &ids[idx..] {
            if count_diff(id1, id2) {
                return filter_diff(id1, id2);
            }
        }
    }

    unreachable!();
}
