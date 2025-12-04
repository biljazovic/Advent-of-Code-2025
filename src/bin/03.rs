advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().filter(|l| !l.is_empty()).map(
        |l| {
            let dgs: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut maxi = 0;
            let mut maxi_i = 0;
            for i in 0..(dgs.len()-1) {
                if dgs[i] > maxi {
                    maxi = dgs[i];
                    maxi_i = i;
                }
            }
            maxi * 10 + *&dgs[maxi_i+1..].iter().max().unwrap()
        }
    ).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input.lines().filter(|l| !l.is_empty()).map(
        |l| {
            let dgs: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut curr_i = 0;
            let mut ret: u64 = 0;
            for k in (0..12).rev() {
                let mut maxi = 0;
                let mut maxi_i = 0;
                for i in curr_i..(dgs.len()-k) {
                    if dgs[i] > maxi {
                        maxi = dgs[i];
                        maxi_i = i;
                    }
                }
                ret = ret * 10 + (maxi as u64);
                curr_i = maxi_i + 1;
            }
            ret
        }
    ).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
