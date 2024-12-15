mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = &args.get(1).expect("HELP: Run with `cargo run -- 01`");

    match day.as_str() {
        "01" => day01::run(),
        "02" => day02::run(),
        "03" => day03::run(),
        "04" => day04::run(),
        "05" => day05::run(),
        "06" => day06::run(),
        "07" => day07::run(),
        "08" => day08::run(),
        "09" => day09::run(),
        "10" => day10::run(),
        "11" => day11::run(),
        "12" => day12::run(),
        "13" => day13::run(),
        "14" => day14::run(),
        "15" => day15::run(),
        _ => unreachable!(),
    }
}
