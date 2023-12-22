mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub struct AOCResult<A> {
    day: u32,
    task: u32,
    r: A,
}

fn measure_time<R: std::fmt::Display, F: FnOnce() -> AOCResult<R>>(f: F) {
    let now = std::time::Instant::now();
    let r = f();
    println!(
        "Day {:2}, Task {}: {:18} in {:12} us",
        r.day,
        r.task,
        r.r,
        now.elapsed().as_micros()
    );
}

fn main() {
    measure_time(day1::task1);
    measure_time(day1::task2);
    measure_time(day2::task1);
    measure_time(day2::task2);
    measure_time(day3::task1);
    measure_time(day3::task2);
    measure_time(day4::task1);
    measure_time(day4::task2);
    measure_time(day5::task1);
    measure_time(day5::task2);
    measure_time(day6::task1);
    measure_time(day6::task2);
    measure_time(day7::task1);
    measure_time(day7::task2);
    measure_time(day8::task1);
    measure_time(day8::task2);
    measure_time(day9::task1);
    measure_time(day9::task2);
    measure_time(day10::task1);
    measure_time(day10::task2);
    measure_time(day11::task1);
    measure_time(day11::task2);
    measure_time(day12::task1);
    measure_time(day12::task2);
    measure_time(day13::task1);
    measure_time(day13::task2);
    measure_time(day14::task1);
    measure_time(day14::task2);
    measure_time(day15::task1);
    measure_time(day15::task2);
    measure_time(day16::task1);
    measure_time(day16::task2);
    measure_time(day17::task1);
    measure_time(day17::task2);
    measure_time(day18::task1);
    measure_time(day18::task2);
    measure_time(day19::task1);
    measure_time(day19::task2);
    measure_time(day20::task1);
    measure_time(day20::task2);
    measure_time(day21::task1);
    measure_time(day21::task2);
    measure_time(day22::task1);
}
