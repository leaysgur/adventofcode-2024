const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    // Find `mul(X,Y)` patterns
    // - X and Y are 1-3 digit numbers
    // - No whitespaces are allowed
    let chars = INPUTS.chars().collect::<Vec<_>>();

    let mut mul_ops = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == 'm' && chars[i + 1] == 'u' && chars[i + 2] == 'l' {
            if chars[i + 3] != '(' {
                i += 1;
                continue;
            }

            let mut j = i + 4;
            let (mut x, mut y) = (0, 0);

            let mut max_digits = 0;
            while chars[j].is_digit(10) && max_digits < 3 {
                x = x * 10 + chars[j].to_digit(10).unwrap();
                j += 1;
                max_digits += 1;
            }

            if chars[j] != ',' {
                i += 1;
                continue;
            }
            j += 1;

            max_digits = 0;
            while chars[j].is_digit(10) && max_digits < 3 {
                y = y * 10 + chars[j].to_digit(10).unwrap();
                j += 1;
            }

            if chars[j] != ')' {
                i += 1;
                continue;
            }

            mul_ops.push((x, y));
        }

        i += 1;
    }
    // println!("mul_ops: {:?}", mul_ops);

    let answer = mul_ops.iter().fold(0, |acc, (x, y)| acc + x * y);
    println!("Answer: {answer}");
}
