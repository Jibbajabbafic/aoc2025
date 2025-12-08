use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(7);

type Position = (usize, usize);

struct Map {
    pub grid: Vec<Vec<char>>,
    pub start: Position,
    pub height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
        let x = grid[0].iter().position(|c| *c == 'S').unwrap();
        let height = grid.len();
        Self {
            grid,
            start: (x, 0),
            height,
        }
    }

    // fn print(&self) {
    //     println!("Grid:");
    //     for line in &self.grid {
    //         println!("{:?}", line);
    //     }
    // }

    fn simulate_p1(&mut self) -> u64 {
        let mut beams: VecDeque<Position> = VecDeque::from([self.start]);
        let mut splits = 0;
        while let Some((x, y)) = beams.pop_front() {
            // self.print();
            let next_y = y + 1;
            // println!("checking {:?} {:?}", x, next_y);
            if next_y >= self.grid.len() {
                // went off map
                continue;
            }
            if self.grid[next_y][x] == '^' {
                // split
                splits += 1;
                if x != 0 && self.grid[next_y][x - 1] == '.' {
                    self.grid[next_y][x - 1] = '|';
                    beams.push_back((x - 1, next_y));
                }
                if x + 1 <= self.grid[next_y].len() && self.grid[next_y][x + 1] == '.' {
                    self.grid[next_y][x + 1] = '|';
                    beams.push_back((x + 1, next_y));
                }
            } else if self.grid[next_y][x] == '.' {
                self.grid[next_y][x] = '|';
                beams.push_back((x, next_y));
            }
        }
        splits
    }

    fn beam(&self, x: usize, y: usize, memo: &mut HashMap<Position, u64>) -> u64 {
        let next_y = y + 1;
        if next_y >= self.height {
            return 1;
        }
        if memo.contains_key(&(x, y)) {
            return *memo.get(&(x, y)).unwrap();
        }
        if self.grid[next_y][x] == '^' {
            // split
            let left = self.beam(x - 1, next_y, memo);
            memo.insert((x - 1, next_y), left);
            let right = self.beam(x + 1, next_y, memo);
            memo.insert((x + 1, next_y), right);
            return left + right;
        } else {
            let result = self.beam(x, next_y, memo);
            memo.insert((x, next_y), result);
            return result;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Map::new(input);
    let result = map.simulate_p1();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::new(input);
    let mut memo = HashMap::new();
    let result = map.beam(map.start.0, map.start.1, &mut memo);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
