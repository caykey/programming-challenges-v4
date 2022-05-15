enum Atom {
    Number(f64),
    Op(String)
}

static OPS: [(&'static str, usize, fn(&[f64]) -> f64); 6] = [
    ("+", 2, |v| v[0] + v[1]),
    ("-", 2, |v| v[0] - v[1]),
    ("*", 2, |v| v[0] * v[1]),
    ("/", 2, |v| v[0] / v[1]),
    ("^", 2, |v| v[0].powf(v[1])),
    ("sqrt", 1, |v| v[0].sqrt()),
];

fn parse(i: &str) -> Vec < Atom > {
    i.split(' ').map(|a| a.trim()).filter(|a| !a.is_empty()).map(|a|
        match a.parse:: < f64 > () {
            Ok(v) => Atom::Number(v),
            Err(_) => Atom::Op(a.into()),
        }).collect()
}

fn op_lookup(stack: &mut Vec < f64 >, fn_op: &str) {
    for op in &OPS {
        if op.0 == fn_op {
            if stack.len() < op.1 {
                eprintln!("Function does not have enough operators, needed {}, found {}", op.1, stack.len());
                return;
            }
            let x = op.2(&stack[stack.len() - op.1..]);
            stack.truncate(stack.len() - op.1);
            stack.push(x);
            return;
        }
    }
    eprintln!("Unknown function ({})", fn_op);
}

fn eval(atoms: &[Atom]) -> f64 {
    let mut stack = Vec::new();
    for atom in atoms {
        match atom {
            Atom::Number(v) => stack.push(*v),
            Atom::Op(o) => op_lookup(&mut stack, o)
        }
    }
    if stack.len() != 1 {
        eprintln!("Warning evaluation stack has {} elements left over, there should be 1. Returning most recent evaluation.", stack.len());
    }
    stack.pop().unwrap_or(0.0)
}

fn main() {
    loop {
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap()
        let atoms = parse(&i);
        println!("{}", eval(&atoms));
    }
}
