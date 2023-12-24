use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug)]
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

fn help(hailstones: &[Hailstone]) {
    let h1 = &hailstones[0];
    let p1 = &h1.start;
    let d1 = &h1.speed;
    let h2 = &hailstones[1];
    let p2 = &h2.start;
    let d2 = &h2.speed;
    let h3 = &hailstones[2];
    let p3 = &h3.start;
    let d3 = &h3.speed;

    /*
     * See https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
     * v x w is the cross product.
     * We are in 3 dimensional space.
     * We want for all i
     *
     *      p + t * d = p_i + t * d_i
     * <=> (p - p_i) = t * (d_i - d)
     * <=> (p - p_i) = -t * (d * d_i)
     *
     * => (p - p_i) x -t * (d - d_i)
     *
     * Since -t is a scalar (p - p_i) and (d - d_i) are parallel to each other. Means we can essentially ignore t
     *
     * => (p - p_i) x (d - d_i) = 0
     *
     * Now, taking two i, for example i = 0 and 1, we can equate:
     * 
     * (p - p_1) x (d - d_1) = (p - p_2) x (d - d_2)
     *
     * This can be tediously resolved to a linear equation of the form A * x = b, where
     * x := (p_x, p_y, p_z, d_x, d_y, d_z)^T
     *
     * The matrix A and vector b are defined below. Note that each pair of indices gives us 3 equations, for 6 unknowns.
     * Therefore we need 2 pairs of points to solve this equation.
     *
     * The way the matrix is defined, the odd indices of A and the even indices of A form the equations for a set of index pairs.
     */
    #[rustfmt::skip]
    let A = [
        [-d1.y + d2.y,  d1.x - d2.x,        0f64, p1.y - p2.y, -p1.x + p2.x,         0f64],
        [-d1.y + d3.y,  d1.x - d3.x,        0f64, p1.y - p3.y, -p1.x + p3.x,         0f64],
        [        0f64, -d1.z + d2.z, d1.y - d2.y,        0f64,  p1.z - p2.z, -p1.y + p2.y],
        [        0f64, -d1.z + d3.z, d1.y - d3.y,        0f64,  p1.z - p3.z, -p1.y + p3.y],
        [-d1.z + d2.z,         0f64, d1.x - d2.x, p1.z - p2.z,         0f64, -p1.x + p2.x],
        [-d1.z + d3.z,         0f64, d1.x - d3.x, p1.z - p3.z,         0f64, -p1.x + p3.x],
    ];

    #[rustfmt::skip]
    let b = [
        p1.y * d1.x - p2.y * d2.x - p1.x * d1.y + p2.x * d2.y,
        p1.y * d1.x - p3.y * d3.x - p1.x * d1.y + p3.x * d3.y,
        p1.z * d1.y - p2.z * d2.y - p1.y * d1.z + p2.y * d2.z,
        p1.z * d1.y - p3.z * d3.y - p1.y * d1.z + p3.y * d3.z,
        p1.z * d1.x - p2.z * d2.x - p1.x * d1.z + p2.x * d2.z,
        p1.z * d1.x - p3.z * d3.x - p1.x * d1.z + p3.x * d3.z,
    ];
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

pub fn task2() -> crate::AOCResult<usize> {
    let hailstones = parse();
    help(&hailstones);

    crate::AOCResult {
        day: 24,
        task: 2,
        r: 0,
    }
}
