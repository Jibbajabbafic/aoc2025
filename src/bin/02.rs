advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    let input = input.trim();
    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let first = first.parse::<u64>().unwrap();
        let last = last.parse::<u64>().unwrap();
        for n in first..=last {
            let nstr = n.to_string();
            if nstr.len() % 2 != 0 {
                continue;
            }
            let (l, r) = nstr.split_at(nstr.len() / 2);
            if l == r {
                invalid_sum += n;
            }
        }
    }
    Some(invalid_sum)
}

fn invalid_pt_2(nstr: &str) -> bool {
    for w in 1..=(nstr.len() / 2) {
        let chars = nstr.chars().collect::<Vec<_>>();
        let mut chunks = chars.chunks(w);
        let pattern = chunks.next().unwrap();
        // println!("Checking pattern {:?} in {:?}", pattern, nstr);
        if chunks.all(|chunk| chunk == pattern) {
            // println!("Found invalid pattern {:?} in {:?}", pattern, nstr);
            return true;
        }
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    let input = input.trim();
    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let first = first.parse::<u64>().unwrap();
        let last = last.parse::<u64>().unwrap();
        for n in first..=last {
            let nstr = n.to_string();
            if invalid_pt_2(&nstr) {
                invalid_sum += n;
            }
        }
    }
    Some(invalid_sum)
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
