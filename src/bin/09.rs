advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: u64,
    y: u64,
}

fn area(a: Pos, b: Pos) -> u64 {
    let dx = b.x.abs_diff(a.x) + 1;
    let dy = b.y.abs_diff(a.y) + 1;
    // println!(
    //     "{:?} {:?} dx={:?} dy={:?} area = {:?}",
    //     a,
    //     b,
    //     dx,
    //     dy,
    //     dx * dy
    // );
    dx * dy
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut largest = 0;
    let mut points: Vec<Pos> = input
        .trim()
        .lines()
        .map(|l| {
            let parts = l.split_once(',').unwrap();
            Pos {
                x: parts.0.parse().unwrap(),
                y: parts.1.parse().unwrap(),
            }
        })
        .collect();
    points.sort_by_key(|p| p.x);
    println!("points {:?}", points);
    for (i, p) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            let a = area(*p, points[j]);
            if a > largest {
                largest = a;
            }
        }
    }
    Some(largest)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points: Vec<Pos> = input
        .trim()
        .lines()
        .map(|l| {
            let parts = l.split_once(',').unwrap();
            Pos {
                x: parts.0.parse().unwrap(),
                y: parts.1.parse().unwrap(),
            }
        })
        .collect();
    points.sort_by_key(|p| p.x);

    let centre = Pos {
        x: points.iter().map(|p| p.x).sum::<u64>() / points.len() as u64,
        y: points.iter().map(|p| p.y).sum::<u64>() / points.len() as u64,
    };
    println!("centre: {:?}", centre);
    // Some(1)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
