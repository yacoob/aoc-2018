extern crate image;
use aoc::*;
use regex::Regex;

// We'll keep our stars and their velocites in four separate vectors. Clumsy, but makes
// calculations of bounding boxes faster.
#[derive(Clone, Debug)]
struct Starfield {
    x: Vec<i32>,
    y: Vec<i32>,
    vx: Vec<i32>,
    vy: Vec<i32>,
}

impl Starfield {
    fn new() -> Starfield {
        // My god, it's so not full of stars!
        Starfield {
            x: vec![],
            y: vec![],
            vx: vec![],
            vy: vec![],
        }
    }
}

fn parse_input(input: &str) -> Starfield {
    let mut starfield = Starfield::new();
    let re = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        starfield.x.push(caps[1].parse().unwrap());
        starfield.y.push(caps[2].parse().unwrap());
        starfield.vx.push(caps[3].parse().unwrap());
        starfield.vy.push(caps[4].parse().unwrap());
    }
    assert!(starfield.x.len() == starfield.y.len());
    assert!(starfield.vx.len() == starfield.vy.len());
    assert!(starfield.x.len() == starfield.vx.len());
    starfield
}

// Iä! Iä! Cthulhu fhtagn!
//
// “When the stars were right, They could plunge from world to world through the sky; but when the
// stars were wrong, They could not live.”
fn make_stars_right(stars: &mut Starfield) -> (i32, &str) {
    let mut previous_bounding_box_size = i64::max_value();
    let mut bounding_box_size = previous_bounding_box_size - 1;
    let mut old_stars = Starfield::new();
    let mut eons = -1;
    // Move stars as long as the bounding box for all of them shrinks.
    while bounding_box_size < previous_bounding_box_size {
        // Tick!
        eons += 1;
        // Save current starfield configuration, in case stars start to diverge in this step.
        old_stars = stars.clone();
        // Move the stars.
        for i in 0..stars.vx.len() {
            stars.x[i] += stars.vx[i];
            stars.y[i] += stars.vy[i];
        }
        // Calculate the bounding box size.
        previous_bounding_box_size = bounding_box_size;
        bounding_box_size =
            i64::from(stars.x.iter().max().unwrap() - stars.x.iter().min().unwrap())
                * i64::from(stars.y.iter().max().unwrap() - stars.y.iter().min().unwrap());
    }
    // Make an image.
    // Bold assumption that we've ended up with positive coords.
    assert!(old_stars.x.iter().all(|&x| x > 0));
    assert!(old_stars.y.iter().all(|&y| y > 0));
    // Normalize the coordinates to shave off left and top borders.
    let x_margin = *old_stars.x.iter().min().unwrap() - 2;
    let y_margin = *old_stars.y.iter().min().unwrap() - 2;
    for i in 0..old_stars.x.len() {
        old_stars.x[i] -= x_margin;
        old_stars.y[i] -= y_margin;
    }
    // Add a bit of margin on the other side as well.
    let mut img = image::RgbImage::new(
        (old_stars.x.iter().max().unwrap() + 3) as u32,
        (old_stars.y.iter().max().unwrap() + 3) as u32,
    );
    // Paint a pixel for every star.
    for i in 0..old_stars.x.len() {
        img.put_pixel(
            old_stars.x[i] as u32,
            old_stars.y[i] as u32,
            image::Rgb([255, 0, 0]),
        );
    }
    let path = "/tmp/aocd9.png";
    img.save(path).unwrap();
    (eons, path)
}

fn main() {
    let mut stars = parse_input(&read_file("inputs/10"));
    let (eons, path) = make_stars_right(&mut stars);
    assert_eq!(eons, 10011);
    println!(
        "You've waited {} eons for stars to be right. Behold: {}.",
        eons, path
    );
}
