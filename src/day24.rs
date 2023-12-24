use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct Hailstone {
    start: Vector,
    speed: Vector,
}

impl Hailstone {
    fn intersects_x_y_at(&self, other: &Self) -> Option<(f64, f64)> {
        let ss = &self.speed;
        let os = &other.speed;
        if ss.x / os.x == ss.y / os.y {
            return None;
        }
        let cx = self.start.x - other.start.x;
        let cy = self.start.y - other.start.y;
        let f_y = ss.y / ss.x;
        let c_0_1 = os.y - (f_y * os.x);
        let cy = cy - (f_y * cx);
        let b = cy / c_0_1;
        let a = (cx - (b * os.x)) / (-ss.x);
        // Corssed in the past for one of the hailstones
        if a < 0f64 || b < 0f64 {
            return None;
        }
        // println!("a: {}, b: {}", a, b);
        // if a == b {
        Some((other.start.x + b * os.x, other.start.y + b * os.y))
        // }
        // return None;
    }
}

fn parse() -> Vec<Hailstone> {
    lines_from_file("src/day24.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Hailstone {
    let mut iter = line.split('@');
    Hailstone {
        start: parse_vec(iter.next().unwrap()),
        speed: parse_vec(iter.next().unwrap()),
    }
}

fn parse_vec(s: &str) -> Vector {
    let mut iter = s.split(',');
    let x = iter.next().unwrap().trim().parse().unwrap();
    let y = iter.next().unwrap().trim().parse().unwrap();
    let z = iter.next().unwrap().trim().parse().unwrap();
    Vector { x, y, z }
}

fn collisions_x_y(hailstones: &[Hailstone]) -> impl '_ + Iterator<Item = (f64, f64)> {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            hailstones
                .iter()
                .skip(i + 1)
                .map(|r| l.intersects_x_y_at(r))
        })
        .flatten()
}

fn count_collisions_in_area<A: Iterator<Item = (f64, f64)>>(
    collisions: A,
    area: (f64, f64),
) -> usize {
    collisions
        .filter(|i| area.0 <= i.0 && i.0 <= area.1 && area.0 <= i.1 && i.1 <= area.1)
        .count()
}

pub fn task1() -> crate::AOCResult<usize> {
    let hailstones = parse();
    let collisions = collisions_x_y(&hailstones);
    // let count = count_collisions_in_area(collisions, (7f64, 27f64));
    let count = count_collisions_in_area(collisions, (200000000000000f64, 400000000000000f64));

    crate::AOCResult {
        day: 24,
        task: 1,
        r: count,
    }
}
