use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::Read;
use std::io::Result;

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

struct StringWrapper<'a>(&'a str);

impl<'a> std::hash::Hash for StringWrapper<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.0.as_bytes());
    }
}

struct AOCHasher(u32);

impl std::hash::Hasher for AOCHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 += *byte as u32;
            self.0 *= 17;
            self.0 &= 0xFF;
        }
    }
}

fn parse() -> Vec<String> {
    read_file_to_string("src/day15.txt")
        .unwrap()
        .split(',')
        .map(|s| s.to_owned())
        .collect()
}

pub fn task1() {
    use std::hash::Hash;
    let s = parse();
    let r: u64 = s
        .iter()
        .map(|s| {
            let mut h = AOCHasher(0);
            let sw = StringWrapper(s);
            sw.hash(&mut h);
            h.finish()
        })
        .sum();

    println!("Day 15, Task 1: {}", r);
}

#[derive(Debug, Clone)]
struct Lens {
    name: String,
    focal_strength: u64,
}

#[derive(Debug)]
struct HM(Vec<Vec<Lens>>);

impl HM {
    fn new() -> Self {
        HM(vec![Vec::with_capacity(10); 256])
    }

    fn swap(&mut self, lens: Lens) {
        let box_num = Self::hash(&lens.name);
        let lens_box = &mut self.0[box_num];
        if let Some(index) = lens_box.iter().position(|l| l.name == lens.name) {
            lens_box[index] = lens;
        } else {
            lens_box.push(lens);
        }
    }

    fn remove(&mut self, name: &str) {
        let box_num = Self::hash(name);
        let lens_box = &mut self.0[box_num];
        lens_box.retain(|lens| lens.name != name);
    }

    fn hash(s: &str) -> usize {
        use std::hash::Hash;
        let mut h = AOCHasher(0);
        let sw = StringWrapper(s);
        sw.hash(&mut h);
        h.finish() as usize
    }

    fn calculate_focusing_power(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(box_num, lens_box)| {
                lens_box.iter().enumerate().map(move |(lens_num, lens)| {
                    (box_num + 1) as u64 * (lens_num + 1) as u64 * lens.focal_strength
                })
            })
            .sum()
    }
}

pub fn task2() {
    let mut hm = HM::new();
    let s = parse();
    s.iter().for_each(|s| {
        if s.ends_with('-') {
            hm.remove(&s[0..(s.len() - 1)])
        } else {
            let mut iter = s.split('=');
            let name = iter.next().unwrap();
            let focal_strength = iter.next().unwrap().parse::<u64>().unwrap();
            hm.swap(Lens {
                name: name.to_owned(),
                focal_strength,
            });
        }
    });

    println!("Day 15, Task 2: {:?}", hm.calculate_focusing_power());
}
