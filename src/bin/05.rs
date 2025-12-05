advent_of_code::solution!(5);

fn is_id_fresh(ranges: &Vec<(u64, u64)>, id: u64) -> bool {
    for (min, max) in ranges.iter() {
        if id >= *min && id <= *max {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let fresh_ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(min, max)| (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()))
        .collect();
    println!("{:?}", fresh_ranges);
    for line in parts[1].lines() {
        let id = line.parse::<u64>().unwrap();
        println!("checking id: {:?}", id);
        if is_id_fresh(&fresh_ranges, id) {
            println!("fresh id: {:?}", id);
            result += 1;
        }
    }
    Some(result)
}

fn combine_ranges(input_range: &mut Vec<(u64, u64)>) -> Option<Vec<(u64, u64)>> {
    let mut combined_ranges = Vec::<(u64, u64)>::new();
    let mut modified = false;
    input_range.sort_by(|a, b| a.0.cmp(&b.0));
    println!("sorted: {:?}", input_range);
    for (min, max) in input_range {
        let mut added = false;
        println!("checking: {:?}-{:?}", min, max);
        for i in 0..combined_ranges.len() {
            println!("checking comb: {:?}", combined_ranges[i]);

            if *min >= combined_ranges[i].0 && *max <= combined_ranges[i].1 {
                // fully inside another range, skip this one
                added = true;
                modified = true;
                println!("skip - inside existing range");
                continue;
            }
            if *min < combined_ranges[i].0 && *max >= combined_ranges[i].0 {
                // Modify min to include this one
                combined_ranges[i].0 = *min;
                println!("new min = {:?}", *min);
                modified = true;
                added = true;
            }
            if *max > combined_ranges[i].1 && *min <= combined_ranges[i].1 {
                // Modified *max to include this one
                combined_ranges[i].1 = *max;
                println!("new max = {:?}", max);
                modified = true;
                added = true;
            }
            if added {
                break;
            }
        }
        if !added {
            combined_ranges.push((*min, *max));
        }
    }
    if modified {
        Some(combined_ranges)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let mut fresh_ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(min, max)| (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()))
        .collect();
    let mut combined_ranges = fresh_ranges;
    loop {
        match combine_ranges(&mut combined_ranges) {
            Some(r) => combined_ranges = r,
            None => break,
        }
    }
    let mut result = 0;
    println!("all combined: {:?}", combined_ranges);
    for (min, max) in combined_ranges {
        result += max - min + 1;
    }

    Some(result)
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

    #[test]
    fn test_part_two_count() {
        let result = part_two(
            "5-10
",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_overlap() {
        let result = part_two(
            "1-9
5-6
2-3
",
        );
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_merging_min_max() {
        let result = part_two(
            "5-10
3-6
8-12
",
        );
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_merging_min_max_exact() {
        let result = part_two(
            "5-10
3-5
10-12
",
        );
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_() {
        let result = part_two(
            "40-45
50-60
80-90
5-20
0-100
6-8
",
        );
        assert_eq!(result, Some(101));
    }
}
