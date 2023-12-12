use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt,
    io::{self, BufRead},
    ops,
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid position format, expected 'x,y,z'")]
    InvalidPositionFormat,
    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Position3d {
    pub fn manhattan_distance(self, other: Position3d) -> isize {
        let d = self - other;
        d.x.abs() + d.y.abs() + d.z.abs()
    }
}

impl ops::Add for Position3d {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl ops::Sub for Position3d {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl FromStr for Position3d {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Position3d, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or(ParseError::InvalidPositionFormat)?
            .parse()?;
        let y = parts
            .next()
            .ok_or(ParseError::InvalidPositionFormat)?
            .parse()?;
        let z = parts
            .next()
            .ok_or(ParseError::InvalidPositionFormat)?
            .parse()?;
        Ok(Self { x, y, z })
    }
}

impl fmt::Display for Position3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Distance3d {
    // Stores the distance in each dimension sorted in ascending order.
    // Sorting has the advantage that the equality and hashing is orientation invariant.
    // Storing individual distances instead of the euclidean distance has the advantage
    // that we can avoid hashing of floating point numbers.
    dists_sorted: [isize; 3],
}

impl Distance3d {
    pub fn new(d1: isize, d2: isize, d3: isize) -> Distance3d {
        let mut dists = [d1, d2, d3];
        dists.sort();
        Self {
            dists_sorted: dists,
        }
    }

    pub fn between(a: &Position3d, b: &Position3d) -> Distance3d {
        Self::new((a.x - b.x).abs(), (a.y - b.y).abs(), (a.z - b.z).abs())
    }

    pub fn euclid(&self) -> f64 {
        self.dists_sorted
            .iter()
            .map(|x| (x * x) as f64)
            .sum::<f64>()
            .sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.dists_sorted.iter().all(|&x| x == 0)
    }
}

impl fmt::Debug for Distance3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Distance3d")
            .field("euclid", &self.euclid())
            .finish()
    }
}

// each beacon to all other beacons
fn compute_all_distances(positions: &[Position3d]) -> Vec<Vec<Distance3d>> {
    positions
        .iter()
        .map(|a| {
            positions
                .iter()
                .map(|b| Distance3d::between(a, b))
                .collect()
        })
        .collect()
}

// allows to look up beacon indices by distance
fn compute_distance_lookup_table(
    all_distances: &[Vec<Distance3d>],
) -> HashMap<Distance3d, Vec<(usize, usize)>> {
    let mut lookup_table: HashMap<_, Vec<_>> = HashMap::new();
    for (i, distances) in all_distances.iter().enumerate() {
        for (j, dist) in distances.iter().enumerate() {
            if dist.is_zero() {
                continue;
            }
            lookup_table.entry(dist.clone()).or_default().push((i, j));
        }
    }
    lookup_table
}

#[derive(Debug, Default)]
struct Map {
    pub positions: Vec<Position3d>,
    pub all_distances: Vec<Vec<Distance3d>>,
    pub distance_to_beacons: HashMap<Distance3d, Vec<(usize, usize)>>,
}

impl Map {
    pub fn new(positions: Vec<Position3d>) -> Map {
        let all_distances = compute_all_distances(&positions);
        let distance_to_beacons = compute_distance_lookup_table(&all_distances);
        Self {
            positions,
            all_distances,
            distance_to_beacons,
        }
    }

    pub fn insert_beacons(&mut self, positions: &[Position3d]) {
        // TODO orientation and offset
        self.positions.extend_from_slice(positions);
        self.positions.sort_unstable();
        self.positions.dedup();

        self.all_distances = compute_all_distances(&self.positions);
        self.distance_to_beacons = compute_distance_lookup_table(&self.all_distances);
    }

    pub fn beacons_count(&self) -> usize {
        self.positions.len()
    }
}

fn subsets_of_length<T: Copy>(length: usize, s: &[T]) -> Vec<Vec<T>> {
    // TODO if copy trait is absent should return references
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .copied()
                .collect()
        })
        .filter(|s: &Vec<T>| s.len() == length)
        .collect()
}

fn possible_matching_beacons(
    distance_to_beacons1: &HashMap<Distance3d, Vec<(usize, usize)>>,
    distance_to_beacons2: &HashMap<Distance3d, Vec<(usize, usize)>>,
) -> (Vec<usize>, Vec<usize>) {
    let mut possible_beacons1 = HashSet::new();
    let mut possible_beacons2 = HashSet::new();

    for (dist1, pairs1) in distance_to_beacons1 {
        if let Some(pairs2) = distance_to_beacons2.get(dist1) {
            for &(s1, t1) in pairs1 {
                possible_beacons1.insert(s1);
                possible_beacons1.insert(t1);
            }
            for &(s2, t2) in pairs2 {
                possible_beacons2.insert(s2);
                possible_beacons2.insert(t2);
            }
        }
    }

    (
        possible_beacons1.into_iter().collect(),
        possible_beacons2.into_iter().collect(),
    )
}

fn beacons_with_distances_sorted(
    all_distances: &[Vec<Distance3d>],
    beacons: &[usize],
) -> (Vec<Vec<Distance3d>>, Vec<usize>) {
    let dinstances_and_beacons: BTreeMap<_, _> = beacons
        .iter()
        .map(|&a| {
            let mut distances = beacons
                .iter()
                .map(|&b| all_distances[a][b])
                .collect::<Vec<_>>();
            distances.sort_unstable();
            (distances, a)
        })
        .collect();
    let distances: Vec<_> = dinstances_and_beacons.keys().cloned().collect();
    let beacons: Vec<_> = dinstances_and_beacons.values().copied().collect();
    (distances, beacons)
}

#[derive(Debug, Clone)]
struct Warp {
    sel: [usize; 3],
    mul: [isize; 3],
    ofs: Position3d,
}

impl Warp {
    pub fn new(sel: [usize; 3], mul: [isize; 3]) -> Warp {
        Self {
            sel,
            mul,
            ofs: Position3d::default(),
        }
    }

    pub fn warp(&self, pos: Position3d) -> Position3d {
        let c = [pos.x, pos.y, pos.z];
        let warped_pos = Position3d {
            x: c[self.sel[0]] * self.mul[0],
            y: c[self.sel[1]] * self.mul[1],
            z: c[self.sel[2]] * self.mul[2],
        };
        warped_pos + self.ofs
    }

    pub fn with_offset(self, ofs: Position3d) -> Warp {
        Self { ofs, ..self }
    }

    pub fn second_to_first(p1: Position3d, p2: Position3d) -> Warp {
        let mut sel = [0, 1, 2];
        let mut mul = [1; 3];

        if p1.x.abs() == p2.x.abs() {
            sel[0] = 0;
            if p1.x != p2.x {
                mul[0] = -1;
            }
        } else if p1.x.abs() == p2.y.abs() {
            sel[0] = 1;
            if p1.x != p2.y {
                mul[0] = -1;
            }
        } else if p1.x.abs() == p2.z.abs() {
            sel[0] = 2;
            if p1.x != p2.z {
                mul[0] = -1;
            }
        }

        if p1.y.abs() == p2.y.abs() {
            sel[1] = 1;
            if p1.y != p2.y {
                mul[1] = -1;
            }
        } else if p1.y.abs() == p2.x.abs() {
            sel[1] = 0;
            if p1.y != p2.x {
                mul[1] = -1;
            }
        } else if p1.y.abs() == p2.z.abs() {
            sel[1] = 2;
            if p1.y != p2.z {
                mul[1] = -1;
            }
        }

        if p1.z.abs() == p2.z.abs() {
            sel[2] = 2;
            if p1.z != p2.z {
                mul[2] = -1;
            }
        } else if p1.z.abs() == p2.x.abs() {
            sel[2] = 0;
            if p1.z != p2.x {
                mul[2] = -1;
            }
        } else if p1.z.abs() == p2.y.abs() {
            sel[2] = 1;
            if p1.z != p2.y {
                mul[2] = -1;
            }
        }

        Warp::new(sel, mul)
    }
}

impl Default for Warp {
    fn default() -> Warp {
        Self::new([0, 1, 2], [1; 3])
    }
}

fn compute_relative_position_and_orientation_between(
    scanner1: &Map,
    scanner2: &Map,
    min_overlap: usize,
) -> Option<(Position3d, Warp)> {
    let (beacons1, beacons2) =
        possible_matching_beacons(&scanner1.distance_to_beacons, &scanner2.distance_to_beacons);

    if beacons1.len() < min_overlap || beacons2.len() < min_overlap {
        return None;
    }

    // FIXME compute possible tranformation matrix between beacons
    // Should map to same origin
    // REMOVE subset

    let beacons1_subsets = subsets_of_length(min_overlap, &beacons1);
    let beacons2_subsets = subsets_of_length(min_overlap, &beacons2);

    let distances_beacons1: Vec<_> = beacons1_subsets
        .into_iter()
        .map(|subset| beacons_with_distances_sorted(&scanner1.all_distances, &subset))
        .collect();

    let distances_beacons2: Vec<_> = beacons2_subsets
        .into_iter()
        .map(|subset| beacons_with_distances_sorted(&scanner2.all_distances, &subset))
        .collect();

    for (distances1, beacons1) in &distances_beacons1 {
        for (distances2, beacons2) in &distances_beacons2 {
            if distances1 == distances2 {
                /*println!("FOUND OVERLAP");
                for &i in &beacons1 {
                    println!("{}", scanner1.positions[i]);
                }
                println!("");
                for &i in &beacons2 {
                    println!("{}", scanner2.positions[i]);
                }*/

                let p1 = {
                    let beacon11 = beacons1[4]; // WTF?
                    let beacon11_pos = scanner1.positions[beacon11];

                    let beacon12 = beacons1[5]; // WTF?
                    let beacon12_pos = scanner1.positions[beacon12];

                    beacon11_pos - beacon12_pos
                };

                let p2 = {
                    let beacon21 = beacons2[4]; // WTF?
                    let beacon21_pos = scanner2.positions[beacon21];

                    let beacon22 = beacons2[5]; // WTF?
                    let beacon22_pos = scanner2.positions[beacon22];

                    beacon21_pos - beacon22_pos
                };

                // TODO find the first number where this holds
                assert_ne!(p1.x, p1.y);
                assert_ne!(p1.y, p1.z);
                assert_ne!(p1.x, p1.z);

                //println!("CHECK {} and {}", p1, p2);
                let warp = Warp::second_to_first(p1, p2);
                // dbg!(&warp);

                let beacon1 = beacons1[0];
                let beacon1_pos = scanner1.positions[beacon1];

                let beacon2 = beacons2[0];
                let beacon2_pos = warp.warp(scanner2.positions[beacon2]);

                let scanner2_pos = beacon1_pos - beacon2_pos;
                return Some((scanner2_pos, warp.with_offset(scanner2_pos)));
            }
        }
    }

    None
}

fn compute_map(scanners: &[Map]) -> (Map, Vec<Position3d>) {
    let mut map = Map::default();

    map.insert_beacons(&scanners[0].positions);

    let mut open: Vec<_> = (1..scanners.len()).collect();

    let mut i = 0;

    let mut scanner_positions = vec![Position3d::default()];

    while let Some(scanner_index) = open.pop() {
        let scanner = &scanners[scanner_index];

        if let Some((scanner_pos, warp)) =
                    compute_relative_position_and_orientation_between(&map, scanner, 12)
                {
                    println!("Found Scanner {} at {}", scanner_index, scanner_pos);
                    let beacons: Vec<_> = scanner.positions.iter().map(|&p| warp.warp(p)).collect();
                    /*for p in &beacons {
                        println!("{}", p);
                    }*/
                    map.insert_beacons(&beacons);
                    scanner_positions.push(scanner_pos);
        } else {
            open.insert(0, scanner_index);
        }

        i += 1;
        if i > 100 {
            panic!("ABORTED");
        }
    }

    (map, scanner_positions)
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|s| s.unwrap()).collect();
    let scanners: Vec<Map> = lines
        .split(|line| line.starts_with("--- scanner "))
        .skip(1)
        .map(|lines| {
            let positions = lines
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            Map::new(positions)
        })
        .collect();

    let (map, scanner_positions) = compute_map(&scanners);
    println!("Part 1: {}", map.beacons_count());

    let max_distance = scanner_positions
        .iter()
        .flat_map(|p1| {
            scanner_positions
                .iter()
                .map(|&p2| p1.manhattan_distance(p2))
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max_distance);

    /*let mut positions = map.positions.clone();
    positions.sort();
    for p in positions {
        println!("{}", p);
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_compute_relative_position_and_orientation_between_2d_example_s2() {
        let positions1 = vec![
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 3, y: 3, z: 0 },
        ];
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            Position3d { x: -1, y: -1, z: 0 },
            Position3d { x: -5, y: 0, z: 0 },
            Position3d { x: -2, y: 1, z: 0 },
        ];
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 3).unwrap();
        assert_eq!(rel_pos, Position3d { x: 5, y: 2, z: 0 });
    }

    #[test]
    fn test_compute_relative_position_and_orientation_between_2d_example_s3() {
        let positions1 = vec![
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 3, y: 3, z: 0 },
        ];
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            Position3d { x: -5, y: 1, z: 0 },
            Position3d { x: -4, y: 5, z: 0 },
            Position3d { x: -3, y: 2, z: 0 },
        ];
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 3).unwrap();
        assert_eq!(rel_pos, Position3d { x: 5, y: 6, z: 0 });
    }

    #[test]
    fn test_compute_relative_position_and_orientation_between_2d_example_s4() {
        let positions1 = vec![
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 3, y: 3, z: 0 },
        ];
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            Position3d { x: -3, y: 3, z: 0 },
            Position3d { x: 1, y: 2, z: 0 },
            Position3d { x: -2, y: 1, z: 0 },
        ];
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 3).unwrap();
        assert_eq!(rel_pos, Position3d { x: 1, y: 4, z: 0 });
    }

    #[test]
    fn test_compute_relative_position_and_orientation_between_2d_example_s5() {
        let positions1 = vec![
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 3, y: 3, z: 0 },
        ];
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            Position3d { x: 3, y: 5, z: 0 },
            Position3d { x: 2, y: 1, z: 0 },
            Position3d { x: 1, y: 4, z: 0 },
        ];
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 3).unwrap();
        assert_eq!(rel_pos, Position3d { x: -1, y: 4, z: 0 });
    }

    #[test]
    fn test_compute_relative_position_and_orientation_between_3d_example_scanner0_scanner1() {
        let positions1 = vec![
            "404,-588,-901",
            "528,-643,409",
            "-838,591,734",
            "390,-675,-793",
            "-537,-823,-458",
            "-485,-357,347",
            "-345,-311,381",
            "-661,-816,-575",
            "-876,649,763",
            "-618,-824,-621",
            "553,345,-567",
            "474,580,667",
            "-447,-329,318",
            "-584,868,-557",
            "544,-627,-890",
            "564,392,-477",
            "455,729,728",
            "-892,524,684",
            "-689,845,-530",
            "423,-701,434",
            "7,-33,-71",
            "630,319,-379",
            "443,580,662",
            "-789,900,-551",
            "459,-707,401",
        ];
        let positions1 = positions1
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390",
        ];
        let positions2 = positions2
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 12).unwrap();
        assert_eq!(
            rel_pos,
            Position3d {
                x: 68,
                y: -1246,
                z: -43,
            }
        );
    }

    #[test]
    fn test_compute_relative_position_and_orientation_between_3d_example_scanner1_scanner4() {
        let positions1 = vec![
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390",
        ];
        let positions1 = positions1
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let scanner1 = Map::new(positions1);

        let positions2 = vec![
            "727,592,562",
            "-293,-554,779",
            "441,611,-461",
            "-714,465,-776",
            "-743,427,-804",
            "-660,-479,-426",
            "832,-632,460",
            "927,-485,-438",
            "408,393,-506",
            "466,436,-512",
            "110,16,151",
            "-258,-428,682",
            "-393,719,612",
            "-211,-452,876",
            "808,-476,-593",
            "-575,615,604",
            "-485,667,467",
            "-680,325,-822",
            "-627,-443,-432",
            "872,-547,-609",
            "833,512,582",
            "807,604,487",
            "839,-516,451",
            "891,-625,532",
            "-652,-548,-490",
            "30,-46,-14",
        ];
        let positions2 = positions2
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let scanner2 = Map::new(positions2);

        let (rel_pos, _) =
            compute_relative_position_and_orientation_between(&scanner1, &scanner2, 12).unwrap();
        assert_eq!(
            rel_pos,
            Position3d {
                x: -20,
                y: -1133,
                z: 1061,
            }
        );
    }

    #[test]
    fn test_compute_all_distances() {
        let positions = [
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 3, y: 3, z: 0 },
        ];

        let distances = compute_all_distances(&positions);
        assert_eq!(
            distances,
            vec![
                vec![
                    Distance3d::between(&positions[0], &positions[0]),
                    Distance3d::between(&positions[0], &positions[1]),
                    Distance3d::between(&positions[0], &positions[2])
                ],
                vec![
                    Distance3d::between(&positions[1], &positions[0]),
                    Distance3d::between(&positions[1], &positions[1]),
                    Distance3d::between(&positions[1], &positions[2])
                ],
                vec![
                    Distance3d::between(&positions[2], &positions[0]),
                    Distance3d::between(&positions[2], &positions[1]),
                    Distance3d::between(&positions[2], &positions[2])
                ],
            ]
        );
    }

    #[test]
    fn test_compute_distance_lookup_table() {
        let positions = [
            Position3d { x: 0, y: 2, z: 0 },
            Position3d { x: 4, y: 1, z: 0 },
            Position3d { x: 8, y: 0, z: 0 },
        ];

        let distances = compute_all_distances(&positions);
        let dist_lookup_table = compute_distance_lookup_table(&distances);
        assert_eq!(
            dist_lookup_table,
            vec![
                (
                    Distance3d::new(4, 1, 0),
                    vec![(0, 1), (1, 0), (1, 2), (2, 1)]
                ),
                (Distance3d::new(8, 2, 0), vec![(0, 2), (2, 0)]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_distance_3d_orientation_invariance() {
        let dist1 = Distance3d::between(
            &Position3d { x: 2, y: 5, z: -1 },
            &Position3d { x: 8, y: -1, z: 4 },
        );
        let dist2 = Distance3d::between(
            &Position3d { x: 8, y: -1, z: 4 },
            &Position3d { x: 2, y: 5, z: -1 },
        );
        let dist3 = Distance3d::between(
            &Position3d { x: 4, y: 2, z: -1 },
            &Position3d { x: -1, y: 8, z: 5 },
        );

        assert_eq!(dist1, dist2);
        assert_eq!(dist2, dist3);

        assert_eq!(calculate_hash(&dist1), calculate_hash(&dist2));
        assert_eq!(calculate_hash(&dist2), calculate_hash(&dist3));
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_subsets_of_length() {
        let v = vec![1, 2, 3];
        assert_eq!(subsets_of_length(1, &v), [[1], [2], [3]]);
        assert_eq!(subsets_of_length(2, &v), [[1, 2], [1, 3], [2, 3]]);
        assert_eq!(subsets_of_length(3, &v), [[1, 2, 3]]);
    }
}
