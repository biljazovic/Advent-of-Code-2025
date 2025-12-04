advent_of_code::solution!(4);

use advent_of_code::{CharMatrix, V2};

pub fn part_one(input: &str) -> Option<u64> {
    let mat = CharMatrix::from_str(input);
    let mut res = 0;
    let (mx, my) = mat.bounds();
    for x in 0..mx {
        for y in 0..my {
            let p: V2 = (x as i32, y as i32);
            if mat.at(p) == Some('@') && mat.susedi8(p).filter(|&c| c == '@').count() < 4 {
                res += 1;
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mat = CharMatrix::from_str(input);
    let mut res = 0;
    let (mx, my) = mat.bounds();
    loop {
        let mut res_temp = 0;
        for x in 0..mx {
            for y in 0..my {
                let p: V2 = (x as i32, y as i32);
                if mat.at(p) == Some('@') && mat.susedi8(p).filter(|&c| c == '@').count() < 4 {
                    res_temp += 1;
                    mat.set(p, '.');
                }
            }
        }
        if res_temp == 0 {
            break
        }
        res += res_temp
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
