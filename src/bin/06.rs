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
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut total = 0;
    let height = grid.len();
    let width = grid[0].len();
    let mut current_stack = Vec::<u64>::new();
    let mut next_op: char = ' ';
    // println!("grid {:?}", grid);

    for x in 0..width {
        let mut next_digit = String::new();
        let mut all_whitespace = true;
        for y in 0..height {
            let c = grid[y][x];
            if c.is_whitespace() {
                continue;
            }
            all_whitespace = false;
            if c.is_numeric() {
                next_digit.push(c);
            } else if c == '*' || c == '+' {
                next_op = c;
            }
        }
        if all_whitespace {
            // seen full column of whitespace - finished this problem
            match next_op {
                '+' => total += current_stack.iter().sum::<u64>(),
                '*' => total += current_stack.iter().product::<u64>(),
                _ => unreachable!(),
            }
            // println!("processed problem");
            current_stack.clear();
            next_op = ' ';
        } else {
            current_stack.push(next_digit.parse::<u64>().unwrap());
            // println!("op: {:?} current stack: {:?}", next_op, current_stack);
        }
    }
    // process final problem
    match next_op {
        '+' => total += current_stack.iter().sum::<u64>(),
        '*' => total += current_stack.iter().product::<u64>(),
        _ => unreachable!(),
    }
    // println!("processed problem");
    Some(total)
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
