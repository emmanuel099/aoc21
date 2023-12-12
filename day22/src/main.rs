use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid cuboid format, expected 'x=10..12,y=10..12,z=10..12'")]
    InvalidCuboidFormat,
    #[error("invalid step format")]
    InvalidStepFormat,
    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3 {
    pub fn with_x(mut self, x: i32) -> Pos3 {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: i32) -> Pos3 {
        self.y = y;
        self
    }

    pub fn with_z(mut self, z: i32) -> Pos3 {
        self.z = z;
        self
    }
}

impl std::fmt::Display for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}/{}/{}]", self.x, self.y, self.z)
    }
}

#[derive(Default, Clone)]
struct Cube {
    cubioids: Vec<Cuboid>,
}

impl Cube {
    pub fn active_cell_count(&self) -> usize {
        self.cubioids.iter().map(Cuboid::cells).sum()
    }

    pub fn union(&mut self, cuboid: Cuboid) {
        self.cut(&cuboid);
        self.cubioids.push(cuboid);
    }

    pub fn cut(&mut self, cuboid: &Cuboid) {
        self.cubioids = self.cubioids.iter().flat_map(|c| c.cut(cuboid)).collect();
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "total cells: {}", self.active_cell_count())?;
        for (i, cuboid) in self.cubioids.iter().enumerate() {
            writeln!(f, "{}: {} [cells: {}]", i, cuboid, cuboid.cells())?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Cuboid {
    pos1: Pos3,
    pos2: Pos3,
}

impl Cuboid {
    pub fn new(pos1: Pos3, pos2: Pos3) -> Cuboid {
        Self { pos1, pos2 }
    }

    pub fn cut(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.overlaps(other) {
            return vec![self.clone()];
        }
        if self.fully_covered_by(other) {
            return vec![];
        }

        let xs = {
            let mut xs = Vec::new();
            if self.pos1.x < other.pos1.x {
                assert!(other.pos1.x <= self.pos2.x);
                xs.push(other.pos1.x);
            }
            if other.pos2.x < self.pos2.x {
                assert!(self.pos1.x <= other.pos2.x);
                xs.push(other.pos2.x);
            }
            xs
        };

        let ys = {
            let mut ys = Vec::new();
            if self.pos1.y < other.pos1.y {
                assert!(other.pos1.y <= self.pos2.y);
                ys.push(other.pos1.y);
            }
            if other.pos2.y < self.pos2.y {
                assert!(self.pos1.y <= other.pos2.y);
                ys.push(other.pos2.y);
            }
            ys
        };

        let zs = {
            let mut zs = Vec::new();
            if self.pos1.z < other.pos1.z {
                assert!(other.pos1.z <= self.pos2.z);
                zs.push(other.pos1.z);
            }
            if other.pos2.z < self.pos2.z {
                assert!(self.pos1.z <= other.pos2.z);
                zs.push(other.pos2.z);
            }
            zs
        };

        self.split_x(xs)
            .iter()
            .flat_map(|c| c.split_y(ys.clone()))
            .flat_map(|c| c.split_z(zs.clone()))
            .filter(|c| !c.fully_covered_by(other))
            .collect()
    }

    fn split_x(&self, mut xs: Vec<i32>) -> Vec<Cuboid> {
        xs.insert(0, self.pos1.x);
        xs.push(self.pos2.x);
        xs.windows(2)
            .map(|x| Cuboid::new(self.pos1.with_x(x[0]), self.pos2.with_x(x[1])))
            .collect()
    }

    fn split_y(&self, mut ys: Vec<i32>) -> Vec<Cuboid> {
        ys.insert(0, self.pos1.y);
        ys.push(self.pos2.y);
        ys.windows(2)
            .map(|y| Cuboid::new(self.pos1.with_y(y[0]), self.pos2.with_y(y[1])))
            .collect()
    }

    fn split_z(&self, mut zs: Vec<i32>) -> Vec<Cuboid> {
        zs.insert(0, self.pos1.z);
        zs.push(self.pos2.z);
        zs.windows(2)
            .map(|z| Cuboid::new(self.pos1.with_z(z[0]), self.pos2.with_z(z[1])))
            .collect()
    }

    fn fully_covered_by(&self, other: &Cuboid) -> bool {
        other.pos1.x <= self.pos1.x
            && self.pos2.x <= other.pos2.x
            && other.pos1.y <= self.pos1.y
            && self.pos2.y <= other.pos2.y
            && other.pos1.z <= self.pos1.z
            && self.pos2.z <= other.pos2.z
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        !(other.pos1.x > self.pos2.x
            || other.pos1.y > self.pos2.y
            || other.pos1.z > self.pos2.z
            || other.pos2.x < self.pos1.x
            || other.pos2.y < self.pos1.y
            || other.pos2.z < self.pos1.z)
    }

    pub fn cells(&self) -> usize {
        ((self.pos2.x - self.pos1.x).abs() as usize)
            * ((self.pos2.y - self.pos1.y).abs() as usize)
            * ((self.pos2.z - self.pos1.z).abs() as usize)
    }
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.pos1.x, self.pos2.x, self.pos1.y, self.pos2.y, self.pos1.z, self.pos2.z
        )
    }
}

impl FromStr for Cuboid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Cuboid, Self::Err> {
        let re = Regex::new(
            r"^x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+),y=(?P<y1>-?\d+)\.\.(?P<y2>-?\d+),z=(?P<z1>-?\d+)\.\.(?P<z2>-?\d+)$",
        )
        .unwrap();
        let caps = re.captures(s).unwrap();

        let p1 = Pos3 {
            x: caps
                .name("x1")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
            y: caps
                .name("y1")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
            z: caps
                .name("z1")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
        };

        let mut p2 = Pos3 {
            x: caps
                .name("x2")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
            y: caps
                .name("y2")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
            z: caps
                .name("z2")
                .ok_or(ParseError::InvalidCuboidFormat)?
                .as_str()
                .parse()?,
        };
        // exclude
        p2.x += 1;
        p2.y += 1;
        p2.z += 1;

        Ok(Self::new(p1, p2))
    }
}

enum Step {
    On(Cuboid),
    Off(Cuboid),
}

impl Step {
    pub fn execute(&self, mut cube: Cube) -> Cube {
        match self {
            Self::On(cuboid) => cube.union(cuboid.clone()),
            Self::Off(cuboid) => cube.cut(cuboid),
        };
        //println!("{}", cube);
        cube
    }

    pub fn ignore_part1(&self) -> bool {
        let cuboid = match self {
            Self::On(cuboid) => cuboid,
            Self::Off(cuboid) => cuboid,
        };
        cuboid.pos1.x < -50
            || cuboid.pos1.y < -50
            || cuboid.pos1.z < -50
            || cuboid.pos1.x > 50
            || cuboid.pos1.y > 50
            || cuboid.pos1.z > 50
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Step, Self::Err> {
        let (cmd, cuboid) = s.split_once(' ').ok_or(ParseError::InvalidStepFormat)?;
        let cuboid = cuboid.parse()?;
        match cmd {
            "on" => Ok(Self::On(cuboid)),
            "off" => Ok(Self::Off(cuboid)),
            _ => Err(ParseError::InvalidStepFormat),
        }
    }
}

pub fn main() {
    let steps: Vec<Step> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse().ok())
        .collect();

    let cube1 = steps
        .iter()
        .filter(|step| !step.ignore_part1())
        .fold(Cube::default(), |cube, step| step.execute(cube));
    println!("Part 1: {}", cube1.active_cell_count());

    let cube2 = steps
        .iter()
        .fold(Cube::default(), |cube2, step| step.execute(cube2));
    println!("Part 2: {}", cube2.active_cell_count());
}
