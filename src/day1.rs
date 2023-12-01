use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

pub fn task1() {
    let c = lines_from_file("src/day1_task1.txt").unwrap()
        .map(|line| {
            line.chars()
                .filter(|c| { c.is_numeric() } )
                .fold((-1, -1), |acc, val| {
                    let value = val.to_digit(10).unwrap() as i32;
                    if acc.0 == -1 { 
                        (value, value)
                    } else {
                        (acc.0, value)
                    }
                })
        }).map(|p| {
            p.0 * 10 + p.1
        }).fold(0, i32::wrapping_add);

    println!("{}", c);
}

fn replace_written_digits(input: &str) -> String {
    let replacements = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = String::new();
    let mut skip = 0;
    for start_index in 0..input.len() {
        let substring = &input[start_index..input.len()];
        for &(digit, value) in &replacements {
            if substring.starts_with(digit) {
                skip = digit.len();
                result.push_str(value);
                break;
            }
        }
        if skip > 0 {
            skip -= 1;
        } else {
            result.push(substring.chars().next().unwrap());
        }
    }

    return result;

    
    // let mut temp = String::new();
    // for ch in input.chars() {
    //     temp.push(ch);
    //     for &(digit, num) in &replacements {
    //         if temp.ends_with(digit) {
    //             let t = temp.replace(digit, num);
    //             result.push_str(&t);
    //             temp.clear();
    //             break;
    //         }
    //     }
    // }
    // result.push_str(&temp);  // append remaining characters
    // result
}

pub fn task2() {
    let c = lines_from_file("src/day1_task1.txt").unwrap()
        .map(|line| {
            replace_written_digits(&line)
                .chars()
                .filter(|c| { c.is_numeric() } )
                .fold((-1, -1), |acc, val| {
                    let value = val.to_digit(10).unwrap() as i32;
                    if acc.0 == -1 { 
                        (value, value)
                    } else {
                        (acc.0, value)
                    }
                })
        }).map(|p| {
            p.0 * 10 + p.1
        }).fold(0, i32::wrapping_add);

    println!("{}", c);
}

#[cfg(test)]
mod test {

    use super::replace_written_digits;

    #[test]
    pub fn test() {
        assert_eq!("21", replace_written_digits("twone"));
        assert_eq!("o9", replace_written_digits("onine"));
        assert_eq!("18", replace_written_digits("oneight"));

        assert_eq!("219", replace_written_digits("two1nine"));
        assert_eq!("823", replace_written_digits("eightwothree"));
        assert_eq!("abc123xyz", replace_written_digits("abcone2threexyz"));
        assert_eq!("x2134", replace_written_digits("xtwone3four"));
        assert_eq!("49872", replace_written_digits("4nineeightseven2"));
        assert_eq!("z18234", replace_written_digits("zoneight234"));
        assert_eq!("7pqrst6teen", replace_written_digits("7pqrstsixteen"));
    }

}
