mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = &args.get(1).expect("HELP: Run with `cargo run -- 01`");

    match day.as_str() {
        "01" => day01::run(),
        "02" => day02::run(),
        _ => unreachable!(),
    }
}
