mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
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
        _ => unreachable!(),
    }
}
