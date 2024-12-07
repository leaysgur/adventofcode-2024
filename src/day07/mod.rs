const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let mut answer = 0;
    'target: for (target, values) in INPUTS.lines().map(|line| {
        let (target, values) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let values = values
            .split(" ")
            .map(|d| d.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        (target, values)
    }) {
        'expr: for expr in generate_exprs(&values) {
            let mut expr_iter = expr.iter();
            let Some(Token::Num(mut v)) = expr_iter.next() else {
                unreachable!()
            };

            while let Some(token) = expr_iter.next() {
                match token {
                    Token::Op(op) => {
                        let Some(Token::Num(next_v)) = expr_iter.next() else {
                            unreachable!()
                        };
                        match op {
                            '+' => v += next_v,
                            '*' => v *= next_v,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }

                // Early return if result will be too big
                if target < v {
                    continue 'expr;
                }
            }

            if v == target {
                answer += target;
                continue 'target;
            }
        }
    }

    println!("Answer: {answer}");
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Num(u64),
    Op(char),
}

fn generate_exprs(values: &Vec<u64>) -> Vec<Vec<Token>> {
    fn bruteforce(
        current_expr: Vec<Token>,
        idx: usize,
        values: &Vec<u64>,
        results: &mut Vec<Vec<Token>>,
    ) {
        if idx == values.len() {
            results.push(current_expr);
            return;
        }

        if idx == 0 {
            bruteforce(vec![Token::Num(values[idx])], idx + 1, &values, results);
        } else {
            for op in &OPS {
                let mut new_expr = current_expr.clone();
                new_expr.push(Token::Op(*op));
                new_expr.push(Token::Num(values[idx]));
                bruteforce(new_expr, idx + 1, &values, results);
            }
        }
    }

    const OPS: [char; 2] = ['*', '+'];
    let mut results = vec![];

    bruteforce(vec![], 0, &values, &mut results);
    results
}
