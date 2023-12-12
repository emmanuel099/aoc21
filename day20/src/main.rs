use std::{
    cmp,
    collections::BTreeSet,
    fmt,
    io::{self, BufRead},
};

#[derive(Default, Debug, Clone)]
struct Image {
    pixels: BTreeSet<(isize, isize)>,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
    negative: bool,
}

impl Image {
    pub fn negative() -> Image {
        Self {
            negative: true,
            ..Self::default()
        }
    }

    pub fn is_lit(&self, x: isize, y: isize) -> bool {
        self.pixels.contains(&(x, y)) ^ self.negative
    }

    fn mark_pixel(&mut self, x: isize, y: isize) {
        if self.pixels.insert((x, y)) {
            self.top_left = (cmp::min(x, self.top_left.0), cmp::min(y, self.top_left.1));
            self.bottom_right = (
                cmp::max(x, self.bottom_right.0),
                cmp::max(y, self.bottom_right.1),
            );
        }
    }

    pub fn lit_pixel(&mut self, x: isize, y: isize) {
        if self.negative {
            return;
        }
        self.mark_pixel(x, y);
    }

    pub fn darken_pixel(&mut self, x: isize, y: isize) {
        if !self.negative {
            return;
        }
        self.mark_pixel(x, y);
    }

    pub fn lit_pixel_count(&self) -> usize {
        if self.negative {
            usize::MAX
        } else {
            self.pixels.len()
        }
    }

    fn square_of_pixels(&self, x: isize, y: isize) -> [bool; 9] {
        let mut square = [false; 9];
        for col in 0..3 {
            for row in 0..3 {
                let index = row * 3 + col;
                square[index] = self.is_lit(x - 1 + col as isize, y - 1 + row as isize);
            }
        }
        square
    }

    fn top_left_with_padding(&self, padding: isize) -> (isize, isize) {
        (self.top_left.0 - padding, self.top_left.1 - padding)
    }

    fn bottom_right_with_padding(&self, padding: isize) -> (isize, isize) {
        (self.bottom_right.0 + padding, self.bottom_right.1 + padding)
    }

    pub fn enhance(&self, setting: &[bool]) -> Image {
        let mut result = if setting[0] ^ self.negative {
            // If first enhancement is #, then this would result in infinitely many #.
            // Therefore we negate the image to keep track of .
            Image::negative()
        } else {
            Image::default()
        };

        let top_left = self.top_left_with_padding(2);
        let bottom_right = self.bottom_right_with_padding(2);

        for y in top_left.1..=bottom_right.1 {
            for x in top_left.0..=bottom_right.0 {
                let pixels = self.square_of_pixels(x, y);
                let number = pixels
                    .iter()
                    .fold(0, |number, &p| (number << 1) | (p as usize));
                //println!("({},{}) -> number {}", x, y, number);
                if setting[number] {
                    result.lit_pixel(x, y);
                } else {
                    result.darken_pixel(x, y);
                }
            }
        }
        println!("{}", &result);

        result
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;

        let top_left = self.top_left_with_padding(2);
        let bottom_right = self.bottom_right_with_padding(2);

        for x in top_left.0..=bottom_right.0 {
            write!(f, "{}", if x == 0 { "|" } else { " " })?;
        }
        writeln!(f)?;

        for y in top_left.1..=bottom_right.1 {
            write!(f, "{}", if y == 0 { "-" } else { " " })?;
            for x in top_left.0..=bottom_right.0 {
                write!(f, "{}", if self.is_lit(x, y) { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|s| s.unwrap()).collect();

    let algorithm_setting: Vec<_> = lines[0].chars().map(|c| c == '#').collect();

    let initial_image =
        lines[2..]
            .iter()
            .enumerate()
            .fold(Image::default(), |mut img, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        img.lit_pixel(x as isize - 1, y as isize);
                    }
                });
                img
            });
    println!("{}", &initial_image);

    let final_image = (0..2).fold(initial_image.clone(), |img, _| {
        img.enhance(&algorithm_setting)
    });
    println!("Part 1: {}", final_image.lit_pixel_count());

    let final_image = (0..50).fold(initial_image, |img, _| img.enhance(&algorithm_setting));
    println!("Part 2: {}", final_image.lit_pixel_count());
}
