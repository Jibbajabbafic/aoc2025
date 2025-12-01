advent_of_code::solution!(1);

struct Dial {
    pos: i32,
    max: i32,
}

impl Dial {
    fn new(start_pos: i32, max: i32) -> Self {
        Dial {
            pos: start_pos,
            max,
        }
    }

    fn turn_right(&mut self, step: i32) -> i32 {
        let mut zeroes = 0;
        println!("Turning right {} steps from {}", step, self.pos);
        for _ in 0..step {
            self.pos += 1;
            if self.pos > self.max {
                zeroes += 1;
                self.pos = 0;
                println!("Crossed zero at position {}", self.pos);
            }
        }
        zeroes
    }

    fn turn_left(&mut self, step: i32) -> i32 {
        let mut zeroes = 0;
        println!("Turning left {} steps from {}", step, self.pos);
        for _ in 0..step {
            self.pos -= 1;
            if self.pos == 0 {
                zeroes += 1;
            }
            if self.pos < 0 {
                self.pos = self.max;
                println!("Crossed zero at position {}", self.pos);
            }
        }
        zeroes
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50, 99);
    let mut zeroes = 0;
    for line in input.lines() {
        let step = line[1..].parse::<i32>().unwrap();
        let crossings = match line.chars().next().unwrap() {
            'R' => dial.turn_right(step),
            'L' => dial.turn_left(step),
            _ => panic!("Invalid direction"),
        };
        if dial.pos == 0 {
            zeroes += 1;
        }
    }
    Some(zeroes as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50, 99);
    let mut zeroes = 0;
    for line in input.lines() {
        let step = line[1..].parse::<i32>().unwrap();
        let crossings = match line.chars().next().unwrap() {
            'R' => dial.turn_right(step),
            'L' => dial.turn_left(step),
            _ => panic!("Invalid direction"),
        };
        zeroes += crossings;
    }
    Some(zeroes as u64)
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
        assert_eq!(result, Some(6));
    }
}
