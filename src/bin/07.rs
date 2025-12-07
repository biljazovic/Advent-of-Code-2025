advent_of_code::solution!(7);

use advent_of_code::CharMatrix;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u64> {
    let mat = CharMatrix::from_str(input);
    let (sx, sy) = mat.map_view()
        .filter_map(|(p, c)| (c == 'S').then(|| p)).next()?;
    let mut beams = HashSet::new(); beams.insert(sy);
    let mut res = 0_usize;
    for x in sx..mat.bounds().0 {
        beams = beams.into_iter().flat_map(|y| -> HashSet<i32> {
            match mat.at((x, y)) {
                Some('^') => {
                    res += 1;
                    HashSet::from_iter(vec![y-1, y+1])
                },
                Some(_) => HashSet::from_iter(vec![y]),
                None => HashSet::new()
            }
        }).collect();
    }
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mat = CharMatrix::from_str(input);
    let (sx, sy) = mat.map_view()
        .filter_map(|(p, c)| (c == 'S').then(|| p)).next()?;
    let mut beams = HashMap::new(); beams.insert(sy, 1);
    for x in sx..mat.bounds().0 {
        beams = beams.into_iter().flat_map(|(y, cnt)| {
            match mat.at((x, y)) {
                Some('^') => {
                    vec![(y-1, cnt), (y+1, cnt)]
                },
                Some(_) => vec![(y, cnt)],
                None => vec![]
            }
        }).fold(HashMap::new(), |mut acc, (y, cnt)| {
            *acc.entry(y).or_insert(0) += cnt;
            acc
        });
    }
    Some(
        beams.values().sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
