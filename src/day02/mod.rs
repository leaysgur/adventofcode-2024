const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let answer = INPUTS
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter(|r| is_safe(r).is_ok())
        .count();

    println!("Answer: {answer}");
}

fn is_safe(report: &[u8]) -> Result<(), String> {
    let dir = if report[0] > report[1] {
        "DECR"
    } else {
        "INCR"
    };

    for pair in report.windows(2) {
        if pair[0] == pair[1] {
            return Err("Neither increasing or decreasing".to_string());
        }
        if 3 < pair[0].abs_diff(pair[1]) {
            return Err("Diff should be 1, 2, or 3".to_string());
        }

        match dir {
            "DECR" => {
                if pair[0] < pair[1] {
                    return Err("Should be decreasing".to_string());
                }
            }
            "INCR" => {
                if pair[0] > pair[1] {
                    return Err("Should be increasing".to_string());
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
