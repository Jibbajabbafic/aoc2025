advent_of_code::solution!(3);

fn find_max_in_range(line: &str, start: usize, end: usize) -> (char, usize) {
    let mut highest = '0';
    let mut index = start;
    for i in start..end {
        let n = line.chars().nth(i).unwrap();
        if n > highest {
            highest = n;
            index = i;
        }
    }
    (highest, index)
}

fn find_max(line: &str, max_len: usize) -> u64 {
    let mut result = String::new();
    let mut last_index = 0;
    for i in 0..max_len {
        let (digit, index) = find_max_in_range(line, last_index, line.len() - max_len + i + 1);
        last_index = index + 1;
        result.push(digit);
    }
    return result.parse::<u64>().unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.trim().lines() {
        sum += find_max(line, 2);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.trim().lines() {
        sum += find_max(line, 12);
    }
    Some(sum)
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
