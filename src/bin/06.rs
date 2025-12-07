advent_of_code::solution!(6);

use advent_of_code::CharMatrix;

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let nums = lines.iter().take(lines.len() - 1).map(|l| {
        l.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect()
    }).collect::<Vec<Vec<_>>>();

    let ops = lines.last().unwrap().split(' ').filter_map(|s| {
        s.chars().last()
    }).collect::<Vec<_>>();

    let n = nums.len();
    let m = nums[0].len();

    let res = (0..m).map(|j| {
        let op = |x, y|
            if ops[j] == '*' { x * y }
            else { x + y };
        (0..n).map(|i| nums[i][j]).reduce(op).unwrap_or(0)
    }).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mat = CharMatrix::from_str(input).transpose(' ').unwrap();
    let mut res = 0_u64;
    let mut nums = Vec::<u64>::new();
    for vi in mat.iter().rev() {
        let num = vi
            .iter()
            .filter_map(|c| c.to_digit(10))
            .reduce(|acc, e| acc * 10 + e);
        let Some(num) = num else { continue }; // wtf
        nums.push(num as u64);
        match vi.last() {
            Some('*') => {
                res += nums.iter().fold(1, |x, &y| x * y);
                nums.clear();
            }
            Some('+') => {
                res += nums.iter().fold(0, |x, &y| x + y);
                nums.clear();
            }
            _ => {}
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
