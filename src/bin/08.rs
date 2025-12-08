use std::{collections::HashSet, fmt::Display};

advent_of_code::solution!(8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos3D {
    x: u64,
    y: u64,
    z: u64,
}

impl std::fmt::Debug for Pos3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Display for Pos3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Pos3D {
    fn from(input: &str) -> Self {
        let mut split = input.split(',').map(|n| n.parse::<u64>().unwrap());
        Self {
            x: split.next().unwrap(),
            y: split.next().unwrap(),
            z: split.next().unwrap(),
        }
    }
}

fn sqr_dist_to(from: Pos3D, to: Pos3D) -> u64 {
    let dx = to.x as i64 - from.x as i64;
    let dy = to.y as i64 - from.y as i64;
    let dz = to.z as i64 - from.z as i64;
    return (dx.pow(2) + dy.pow(2) + dz.pow(2)) as u64;
}

type Pair = (Pos3D, Pos3D);

pub fn do_part_one(connections: usize, input: &str) -> Option<u64> {
    let points: Vec<Pos3D> = input.trim().lines().map(|l| Pos3D::from(l)).collect();
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((points[i], points[j]));
        }
    }
    // println!("points {:?} paris {:?}", points.len(), pairs.len());
    pairs.sort_by_key(|a| sqr_dist_to(a.0, a.1));
    // println!("{:#?}", pairs);
    let mut circuits: Vec<HashSet<Pos3D>> = Vec::new();
    // Connect the closest connections up to the input value
    for i in 0..connections {
        if let Some(p) = pairs.get(i) {
            // println!("checking: {:?}", p);
            let mut added = Vec::<usize>::new();
            for j in 0..circuits.len() {
                let c = &mut circuits[j];
                let contains_first = c.contains(&p.0);
                let contains_second = c.contains(&p.1);
                if contains_first && contains_second {
                    // Both already connected
                    // println!("both already in {}", j);
                    added.push(j);
                } else if contains_first && !contains_second {
                    c.insert(p.1);
                    added.push(j);
                } else if !contains_first && contains_second {
                    c.insert(p.0);
                    added.push(j);
                }
            }
            // println!("added: {:?}", added);
            assert!(added.len() <= 2, "Too many circuits to merge?");
            if added.is_empty() {
                // no connection - new circuit
                let mut new_c = HashSet::<Pos3D>::new();
                new_c.insert(p.0);
                new_c.insert(p.1);
                circuits.push(new_c);
                // println!("new circuit");
            } else if added.len() > 1 {
                // need to merge 2 circuits now
                // added[1] index should always be higher since it's populated 2nd
                let second = circuits.remove(added[1]);
                let first = &mut circuits[added[0]];
                first.extend(second);
            }
        } else {
            // All pairs connected
            break;
        }
    }
    // Multiply the top 3 circuit sizes
    circuits.sort_by_key(|c| c.len());
    circuits.reverse();
    // println!("circuits: {:?}", circuits);
    let result = circuits.iter().take(3).map(|c| c.len() as u64).product();
    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    do_part_one(1000, input)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Pos3D> = input.trim().lines().map(|l| Pos3D::from(l)).collect();
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((points[i], points[j]));
        }
    }
    // println!("points {:?} paris {:?}", points.len(), pairs.len());
    pairs.sort_by_key(|a| sqr_dist_to(a.0, a.1));
    // println!("{:#?}", pairs);
    let mut circuits: Vec<HashSet<Pos3D>> = Vec::new();
    // Connect all connections
    let mut last_connected: Pair = pairs[0];
    for p in pairs {
        // println!("checking: {:?}", p);
        let mut added = Vec::<usize>::new();
        for j in 0..circuits.len() {
            let c = &mut circuits[j];
            let contains_first = c.contains(&p.0);
            let contains_second = c.contains(&p.1);
            if contains_first && contains_second {
                // Both already connected
                // println!("both already in {}", j);
                added.push(j);
            } else if contains_first && !contains_second {
                c.insert(p.1);
                added.push(j);
                last_connected = p;
            } else if !contains_first && contains_second {
                c.insert(p.0);
                added.push(j);
                last_connected = p;
            }
        }
        // println!("added: {:?}", added);
        assert!(added.len() <= 2, "Too many circuits to merge?");
        if added.is_empty() {
            // no connection - new circuit
            let mut new_c = HashSet::<Pos3D>::new();
            new_c.insert(p.0);
            new_c.insert(p.1);
            circuits.push(new_c);
            // println!("new circuit");
        } else if added.len() > 1 {
            // need to merge 2 circuits now
            // added[1] index should always be higher since it's populated 2nd
            let second = circuits.remove(added[1]);
            let first = &mut circuits[added[0]];
            first.extend(second);
        }
    }

    // // Multiply the top 3 circuit sizes
    // circuits.sort_by_key(|c| c.len());
    // circuits.reverse();
    // // println!("circuits: {:?}", circuits);
    // let result = circuits.iter().take(3).map(|c| c.len() as u64).product();
    let result = last_connected.0.x * last_connected.1.x;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = do_part_one(10, &advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
