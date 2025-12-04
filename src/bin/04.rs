advent_of_code::solution!(4);

fn count_neighbours(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let mut count = 0;
    let dirs = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for d in dirs {
        let dy = y as isize + d[0];
        let dx = x as isize + d[1];
        if dy < 0 || dy >= grid.len() as isize {
            continue;
        }
        if dx < 0 || dx >= grid[y].len() as isize {
            continue;
        }
        if grid[dy as usize][dx as usize] == '@' {
            count += 1;
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let grid: Vec<Vec<char>> = input.trim().lines().map(|x| x.chars().collect()).collect();
    for y in 0..grid.len() {
        // println!("{:?}", grid[y]);
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }
            if count_neighbours(&grid, x, y) < 4 {
                // println!("found at x={:?} y={:?}", x, y);
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut grid: Vec<Vec<char>> = input.trim().lines().map(|x| x.chars().collect()).collect();
    let mut removed = true;

    while removed {
        removed = false;
        for y in 0..grid.len() {
            // println!("{:?}", grid[y]);
            for x in 0..grid[y].len() {
                if grid[y][x] != '@' {
                    continue;
                }
                if count_neighbours(&grid, x, y) < 4 {
                    grid[y][x] = 'x';
                    result += 1;
                    removed = true;
                }
            }
        }
    }
    Some(result)
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
