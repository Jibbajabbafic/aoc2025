advent_of_code::solution!(6);

fn crank(op: &str, stacks: &mut u64, input: u64) {
    match op {
        "+" => *stacks += input,
        "*" => *stacks *= input,
        _ => panic!("Unknown char: {:?}", op),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.trim().lines().rev();
    let ops: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();
    let mut stacks: Vec<u64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    for line in lines {
        let nums: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        for i in 0..nums.len() {
            crank(ops[i], &mut stacks[i], nums[i]);
        }
    }
    let result = stacks.iter().sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
