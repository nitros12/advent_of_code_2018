use aoc_runner_derive::{aoc, aoc_generator};

use std::{
    fmt::Debug,
    collections::HashMap,
    hash::{Hash, Hasher},
};
use cgmath::prelude::*;
use cgmath::Point2;
use spade::{SpatialObject, BoundingRect, PointN, rtree::RTree};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TaxicabSpatial(pub Point2<i32>);

impl TaxicabSpatial {
    fn new(x: i32, y: i32) -> TaxicabSpatial {
        TaxicabSpatial(Point2::new(x, y))
    }
}

fn taxicab(a: Point2<i32>, b: Point2<i32>) -> i32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()).pow(2)
}

impl SpatialObject for TaxicabSpatial {
    type Point = Point2<i32>;

    fn mbr(&self) -> BoundingRect<Self::Point> {
        self.0.mbr()
    }

    fn distance2(&self, point: &Self::Point) -> i32 {
        taxicab(self.0, *point)
    }
}

lazy_static! {
    static ref COORD_RE: Regex = Regex::new(r"(?P<x>\d+),\s*(?P<y>\d+)").unwrap();
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<TaxicabSpatial> {
    COORD_RE.captures_iter(input)
        .map(|c| TaxicabSpatial::new(
            c["x"].parse::<i32>().unwrap(),
            c["y"].parse::<i32>().unwrap()))
        .collect()
}

#[derive(Default, Debug)]
struct Region {
    members: Vec<TaxicabSpatial>,
    infinite: bool,
}

#[aoc(day6, part1)]
pub fn part1(inp: &[TaxicabSpatial]) -> usize {
    let tree = RTree::bulk_load(inp.to_vec());

    let mut regions: HashMap<TaxicabSpatial, Region> = HashMap::new();

    let bounds = tree.mbr().unwrap();
    let lower = bounds.lower();
    let upper = bounds.upper();

    for y in lower.y - 1 .. upper.y + 2 {
        for x in lower.x - 1 .. upper.x + 2 {
            let point = TaxicabSpatial::new(x, y);
            let closest_n = tree.nearest_n_neighbors(&point.0, 2);

            if closest_n.len() == 2 && taxicab(point.0, closest_n[0].0) == taxicab(point.0, closest_n[1].0) {
                continue;
            }

            let closest = closest_n[0];

            let region = regions.entry(*closest)
                                .or_default();

            region.members.push(point);

            if x < lower.x
                || x > upper.x
                || y < lower.y
                || y > upper.y {
                    region.infinite = true;
                }

        }
    }

    regions.into_iter()
           .filter_map(
               |(_k, v)|
               if v.infinite {
                   None
               } else {
                   Some(v.members.len())
               }
           )
           .max()
           .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1};

    #[test]
    fn t1() {
        let inp = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;

        let parsed = parse_input(inp);

        let result = part1(&parsed);

        assert_eq!(result, 17);
    }
}
