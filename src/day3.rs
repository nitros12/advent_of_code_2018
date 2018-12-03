use aoc_runner_derive::{aoc, aoc_generator, Runner};
use lazy_static::lazy_static;
use ndarray::{s, Array2};
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref CLAIM_RE: Regex = Regex::new(
        r"#(?P<id>\d+) @ (?P<loffset>\d+),(?P<toffset>\d+): (?P<width>\d+)x(?P<height>\d+)"
    )
    .unwrap();
}

pub struct Rect {
    id: u16,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rect> {
    input
        .lines()
        .filter_map(|l| {
            let caps = CLAIM_RE.captures(l)?;

            Some(Rect {
                id: caps.name("id")?.as_str().parse().ok()?,
                x: caps.name("loffset")?.as_str().parse().ok()?,
                y: caps.name("toffset")?.as_str().parse().ok()?,
                w: caps.name("width")?.as_str().parse().ok()?,
                h: caps.name("height")?.as_str().parse().ok()?,
            })
        })
        .collect()
}

impl Rect {
    // [x_min, y_min, x_max, y_max]
    fn to_points(&self) -> (usize, usize, usize, usize) {
        (
            self.x as usize,
            self.y as usize,
            (self.x + self.w) as usize,
            (self.y + self.h) as usize,
        )
    }
}

fn get_min_max(input: &[Rect]) -> (usize, usize, usize, usize) {
    let mut iter = input.iter();

    let first = iter.next().unwrap();

    iter.fold(first.to_points(), |mut minmax, rect| {
        let points = rect.to_points();

        if points.0 < minmax.0 {
            minmax.0 = points.0;
        }
        if points.1 < minmax.1 {
            minmax.1 = points.1;
        }
        if points.2 > minmax.2 {
            minmax.2 = points.2;
        }
        if points.3 > minmax.3 {
            minmax.3 = points.3;
        }

        minmax
    })
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rect]) -> usize {
    let (_min_x, _min_y, max_x, max_y) = get_min_max(input);

    let mut arr = Array2::<usize>::zeros((max_x, max_y));

    for rect in input {
        let (min_x, min_y, max_x, max_y) = rect.to_points();
        let mut arrslice = arr.slice_mut(s![min_x..max_x, min_y..max_y]);
        arrslice += 1;
    }

    arr.mapv_inplace(|x| (x > 1) as usize);
    arr.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Rect]) -> u16 {
    let (_min_x, _min_y, max_x, max_y) = get_min_max(input);

    let mut arr = Array2::<usize>::zeros((max_x, max_y));

    let mut non_overlapping: HashSet<_> = input.iter().map(|r| r.id).collect();

    for rect in input {
        let (min_x, min_y, max_x, max_y) = rect.to_points();
        let mut arrslice = arr.slice_mut(s![min_x..max_x, min_y..max_y]);

        for &val in &arrslice {
            if val != 0 {
                non_overlapping.remove(&(val as u16));
                non_overlapping.remove(&rect.id);
            }
        }

        arrslice.fill(rect.id as usize);
    }

    *non_overlapping.iter().next().unwrap()
}
