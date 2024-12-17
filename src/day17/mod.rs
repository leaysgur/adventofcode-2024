const INPUTS: &str = include_str!("./inputs.txt");

pub fn run() {
    let (registers, program) = INPUTS.split_once("\n\n").unwrap();
    let mut registers = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap());

    let programs = program
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|d| d.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let programs = programs.chunks(2).collect::<Vec<_>>();
    println!("Programs: {programs:?}");

    let mut a = registers.next().unwrap();
    let mut b = registers.next().unwrap();
    let mut c = registers.next().unwrap();
    println!("Registers: a: {a}, b: {b}, c: {c}");

    let operand_to_value = |operand: i64, abc: (i64, i64, i64)| -> i64 {
        match operand {
            4 => abc.0,
            5 => abc.1,
            6 => abc.2,
            _ => operand,
        }
    };

    let mut outputs = vec![];
    let mut ptr = 0;
    loop {
        if ptr >= programs.len() {
            break;
        }

        let inst = programs[ptr];

        let opcode = inst[0];
        let operand = inst[1];
        let combo = operand_to_value(operand, (a, b, c));
        println!("ptr: {ptr}, opcode: {opcode}, operand: {operand}");
        println!("combo: {combo}, a: {a}, b: {b}, c: {c}");

        match opcode {
            // adv
            0 => {
                a = a / (1 << combo);
            }
            // bxl
            1 => {
                b = b ^ operand;
            }
            // bst
            2 => {
                b = combo % 8;
            }
            // jnz
            3 => {
                if a != 0 {
                    ptr = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => {
                b = b ^ c;
            }
            // out
            5 => {
                let out = combo % 8;
                outputs.push(out);
            }
            // bdv
            6 => {
                b = a / (1 << combo);
            }
            // cdv
            7 => {
                c = a / (1 << combo);
            }
            _ => unreachable!("Unknown opcode: {opcode}"),
        }
        ptr += 1;
    }

    let answer = outputs
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("Answer: {answer}");
}
