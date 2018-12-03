use aoc_runner_derive::{aoc, aoc_generator, Runner};
use regex::Regex;
use lazy_static::lazy_static;
use ndarray::{Array2, s};
use ndarray_parallel::prelude::*;
use rayon::prelude::*;
use itertools::Itertools;

lazy_static! {
    static ref CLAIM_RE: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<loffset>\d+),(?P<toffset>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
}

pub struct Rect {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
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
            (self.x, self.y,
             self.x + self.w,
             self.y + self.h)
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rect]) -> usize {

    let (min_x, min_y, max_x, max_y) = {
        let mut iter = input.iter();

        let first = iter.next().unwrap();

        iter.fold(first.to_points(),
                  |mut minmax, rect| {
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
    };

    let mut arr = Array2::<usize>::zeros((max_x, max_y));

    for rect in input {
        let (min_x, min_y, max_x, max_y) = rect.to_points();
        let mut arrslice = arr.slice_mut(s![min_x..max_x, min_y..max_y]);
        arrslice += 1;
    }

    arr.mapv_inplace(|x| if x >= 2 { 1 } else { 0 });
    arr.sum()
}
