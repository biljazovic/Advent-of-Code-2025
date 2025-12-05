advent_of_code::solution!(5);

use std::ops::RangeInclusive;
use rangetools::{Rangetools, BoundedSet};

fn parse(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let ranges = input.lines().take_while(|l| !l.is_empty())
        .filter_map(|l| {
            let (a, b) = l.split_once('-')?;
            let a_num = a.parse::<i64>().ok()?;
            let b_num = b.parse::<i64>().ok()?;
            Some (a_num..=b_num)
        })
        .collect::<Vec<RangeInclusive<i64>>>();
    let ids = input.lines().skip(ranges.len() + 1)
        .filter_map(|l| {
            l.parse::<i64>().ok()
        })
        .collect();
    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse(input);
    Some(ids.iter().filter(|x| ranges.iter().any(|r| r.contains(x))).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);
    let range_union = ranges.into_iter().fold(BoundedSet::empty(), |acc, e| acc.union(e));
    Some(range_union.into_iter().size_hint().0 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
