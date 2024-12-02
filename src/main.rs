mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = &args.get(1).expect("HELP: Run with `cargo run -- 01`");

    match day.as_str() {
        "01" => day01::run(),
        "02" => day02::run(),
        "03" => day03::run(),
        _ => unreachable!(),
    }
}
