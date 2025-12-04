advent_of_code::solution!(2);

pub fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> {
    input
        .lines()
        .next().unwrap()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|par| {
            let (x, y) = par.split_once('-').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
}

pub fn part_one(input: &str) -> Option<i64> {
    // let tot_f = |x| -> i64 {
    //     let digits : Vec<i64> = (0..).scan(x, |x, _| {
    //         if *x == 0 {
    //             return None;
    //         }
    //         let ret = *x % 10;
    //         *x = *x / 10;
    //         Some(ret)
    //     }).collect::<Vec<i64>>().into_iter().rev().collect();
    //     let len2 = digits.len() / 2;
    //     let mut ret = 0_i64;
    //     ret += 10_i64.pow(len2 as u32) - 1; // 9 + 90 + 900 + ...
    //     if digits.len() % 2 == 0 {
    //         if &digits[..len2] <= &digits[len2..] {
    //             ret += 1;
    //         }
    //         ret += digits[..len2].iter().rev()
    //             .fold(0, |acc, &d| acc * 10 + (d - 1))
    //     }
    //     ret
    // };
    let f = |(x, y)| {
        (x..=y).filter(|&x| {
            let mut len = 0;
            let mut y = x;
            while y > 0 {
                len += 1;
                y /= 10;
            }
            if len % 2 == 1 {
                return false
            }
            let p = 10_i64.pow(len / 2);
            return x % p == x / p;
        }).sum::<i64>()
    };
    Some(parse(input).map(f).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let f = |(x, y)| {
        (x..=y).filter(|&x: &i64| {
            let lenc = x.to_string().len();

            for l in 1..=(lenc / 2) {
                if lenc % l > 0 { continue }
                let mut y = x;
                let mut found = true;
                let pow10 = 10_i64.pow(l as u32);
                let ost = y % pow10;
                while y > 0 {
                    found = found && (y % pow10 == ost);
                    y /= pow10;
                }
                if found {
                    return true;
                }
            }
            false
        }).sum::<i64>()
    };
    Some(parse(input).map(f).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
