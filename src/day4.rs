use aoc_runner_derive::{aoc, aoc_generator, Runner};

use chrono::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::{s, Array1, Array2};
use regex::Regex;

lazy_static! {
    static ref PARTIAL_EVENT_RE: Regex =
        Regex::new(r"\[(?P<datetime>[^\]]+)\]\s*(?P<event>.+)").unwrap();
    static ref EVENT_RE: Regex = Regex::new(concat!(
        r"(?P<begin>Guard #(?P<guard_id>\d+) begins shift)|",
        r"(?P<wakeup>wakes up)|",
        r"(?P<sleep>falls asleep)"
    ))
    .unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum EventType {
    WakeUp,
    Asleep,
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    datetime: NaiveDateTime,
    guard_id: usize,
    event_type: EventType,
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Event> {
    #[derive(Debug)]
    struct PartialEvent<'a> {
        datetime: NaiveDateTime,
        event: &'a str,
    }

    let mut events: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let caps = PARTIAL_EVENT_RE.captures(l)?;

            Some(PartialEvent {
                datetime: NaiveDateTime::parse_from_str(
                    caps.name("datetime")?.as_str(),
                    "%Y-%m-%d %H:%M",
                )
                .ok()?,
                event: caps.name("event")?.as_str(),
            })
        })
        .collect();

    events.sort_by_key(|e| e.datetime);

    events
        .iter()
        .scan(None, |current_guard, l| {
            let caps = EVENT_RE.captures(l.event)?;

            let event_type = if caps.name("begin").is_some() {
                *current_guard = Some(caps.name("guard_id")?.as_str().parse().ok()?);

                return Some(None);
            } else if caps.name("wakeup").is_some() {
                EventType::WakeUp
            } else {
                EventType::Asleep
            };

            Some(Some(Event {
                datetime: l.datetime,
                guard_id: (*current_guard)?,
                event_type,
            }))
        })
        .filter_map(|i| i)
        .collect()
}

fn as_sleep_periods<'a>(
    inp: &'a [Event],
) -> impl Iterator<Item = (usize, NaiveDateTime, i64)> + 'a {
    inp.iter().tuples().map(|(l, r)| {
        assert_eq!(l.guard_id, r.guard_id);
        assert_eq!(l.event_type, EventType::Asleep);
        assert_eq!(r.event_type, EventType::WakeUp);

        (
            l.guard_id,
            l.datetime,
            (r.datetime - l.datetime).num_minutes(),
        )
    })
}

#[aoc(day4, part1)]
pub fn part1(inp: &[Event]) -> usize {
    let guards_periods = as_sleep_periods(inp)
        .map(|(id, s, d)| (id, (s, d)))
        .into_group_map();

    let (most_asleep_guard, most_asleep_minute, _) = guards_periods.iter().fold(
        (0, None, 0),
        |(max_guard, max_minute, max_duration), (guard, periods)| {
            let mut arr = Array1::<u8>::zeros(60);

            let mut minutes_asleep = 0;

            for period in periods {
                let min_bound = period.0.time().minute() as usize;
                let max_bound = min_bound + period.1 as usize;
                let mut slice = arr.slice_mut(s![min_bound..max_bound]);

                minutes_asleep += period.1;

                slice += 1;
            }

            let (max_index, _) = arr.into_iter().enumerate().max_by_key(|(_, n)| *n).unwrap();

            if minutes_asleep > max_duration {
                (*guard, Some(max_index), minutes_asleep)
            } else {
                (max_guard, max_minute, max_duration)
            }
        },
    );

    return most_asleep_minute.unwrap() as usize * most_asleep_guard;
}

// returns maximum and second maximum
fn max_2_by_key<T, U: Ord>(
    it: impl IntoIterator<Item = T>,
    fun: impl Fn(&T) -> U,
) -> Option<(T, T)> {
    let mut it = it.into_iter();

    let mut max = it.next()?;
    let mut max_val = fun(&max);
    let mut max_2 = it.next()?;
    let mut max_2_val = fun(&max_2);

    if max_2_val > max_val {
        std::mem::swap(&mut max, &mut max_2);
        std::mem::swap(&mut max_val, &mut max_2_val);
    }

    for i in it {
        let ii = fun(&i);
        if ii > max_val {
            max = i;
            max_val = ii;
        } else if ii > max_2_val {
            max_2 = i;
            max_2_val = ii;
        }
    }

    Some((max, max_2))
}

#[aoc(day4, part2)]
pub fn part2(inp: &[Event]) -> usize {
    use bimap::BiMap;

    let guards_periods = as_sleep_periods(inp)
        .map(|(id, s, d)| (id, (s, d)))
        .into_group_map();

    let guard_idx_ids: BiMap<_, _> = guards_periods.keys().enumerate().collect();

    let mut arrs = Array2::<u8>::zeros((guards_periods.len(), 60));

    for (guard, periods) in &guards_periods {
        let idx = guard_idx_ids.get_by_right(&guard).unwrap();
        let mut arr = arrs.row_mut(*idx);

        for period in periods {
            let min_bound = period.0.time().minute() as usize;
            let max_bound = min_bound + period.1 as usize;
            let mut slice = arr.slice_mut(s![min_bound..max_bound]);

            slice += 1;
        }
    }

    let (col_idx, max_idx, _) = arrs
        .gencolumns()
        .into_iter()
        .enumerate()
        .map(|(col_idx, col)| {
            let ((max_idx, max_val), (_, max_2_val)) =
                max_2_by_key(col.iter().enumerate(), |(_, n)| *n).unwrap();

            (col_idx, max_idx, max_val - max_2_val)
        })
        .max_by_key(|(_, _, diff)| *diff)
        .unwrap();

    let guard_id = guard_idx_ids.get_by_left(&max_idx).unwrap();

    return *guard_id * col_idx;
}
