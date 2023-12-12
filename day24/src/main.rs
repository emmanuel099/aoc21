use std::fs::File;
use std::io::prelude::*;
use std::{
    collections::HashMap,
    io::{self, BufRead},
    str,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Variable(Variable),
    Literal(i64),
}

impl Operand {
    pub fn read_vars(self) -> Vec<Variable> {
        match self {
            Self::Variable(var) => {
                vec![var]
            }
            _ => vec![],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

impl Instruction {
    pub fn read_vars(self) -> Vec<Variable> {
        match self {
            Instruction::Inp(_) | Instruction::Mul(_, Operand::Literal(0)) => {
                vec![]
            }
            Instruction::Add(a, b)
            | Instruction::Mul(a, b)
            | Instruction::Div(a, b)
            | Instruction::Mod(a, b)
            | Instruction::Eql(a, b) => {
                let mut vars = b.read_vars();
                vars.push(a);
                vars
            }
        }
    }

    pub fn written_vars(self) -> Vec<Variable> {
        match self {
            Instruction::Inp(a)
            | Instruction::Add(a, _)
            | Instruction::Mul(a, _)
            | Instruction::Div(a, _)
            | Instruction::Mod(a, _)
            | Instruction::Eql(a, _) => {
                vec![a]
            }
        }
    }

    fn parse_var(mut chars: str::Chars<'_>) -> (str::Chars<'_>, Variable) {
        let var = match chars.next() {
            Some('w') => Variable::W,
            Some('x') => Variable::X,
            Some('y') => Variable::Y,
            Some('z') => Variable::Z,
            _ => panic!(),
        };
        (chars, var)
    }

    fn parse_number(mut chars: str::Chars<'_>) -> (str::Chars<'_>, i64) {
        let s = chars.as_str();
        while chars
            .clone()
            .next()
            .map_or(false, |c| c.is_numeric() || c == '-')
        {
            chars.next();
        }
        let n = &s[..s.len() - chars.as_str().len()];
        (chars, n.parse().unwrap())
    }

    fn parse_operand(mut chars: str::Chars<'_>) -> (str::Chars<'_>, Operand) {
        match chars.clone().next() {
            Some(c) if c.is_numeric() || c == '-' => {
                let (chars, n) = Self::parse_number(chars);
                (chars, Operand::Literal(n))
            }
            Some(c) => {
                let (chars, var) = Self::parse_var(chars);
                (chars, Operand::Variable(var))
            }
            _ => panic!(),
        }
    }

    fn parse_identifier(mut chars: str::Chars<'_>) -> (str::Chars<'_>, &str) {
        let s = chars.as_str();
        while chars.clone().next().map_or(false, |c| !c.is_whitespace()) {
            chars.next();
        }
        let n = &s[..s.len() - chars.as_str().len()];
        (chars, n)
    }

    fn parse_instruction(mut chars: str::Chars<'_>) -> (str::Chars<'_>, Instruction) {
        let (mut chars, ident) = Self::parse_identifier(chars);
        match ident {
            "inp" => {
                chars.next(); // space
                let (chars, a) = Self::parse_var(chars);
                (chars, Instruction::Inp(a))
            }
            "add" | "mul" | "div" | "mod" | "eql" => {
                chars.next(); // space
                let (mut chars, a) = Self::parse_var(chars);
                chars.next(); // space
                let (chars, b) = Self::parse_operand(chars);
                (
                    chars,
                    match ident {
                        "add" => Instruction::Add(a, b),
                        "mul" => Instruction::Mul(a, b),
                        "div" => Instruction::Div(a, b),
                        "mod" => Instruction::Mod(a, b),
                        "eql" => Instruction::Eql(a, b),
                        _ => panic!(),
                    },
                )
            }
            _ => panic!(),
        }
    }

    pub fn parse(s: &str) -> Instruction {
        let (_, inst) = Self::parse_instruction(s.chars());
        inst
    }
}

trait Port {
    fn next(&mut self) -> i64;
}

struct ALU<'port, InputPort> {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input: &'port mut InputPort,
}

impl<'port, InputPort: Port> ALU<'port, InputPort> {
    pub fn new(input: &'port mut InputPort) -> ALU<'port, InputPort> {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input,
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|inst| self.dispatch(inst));
    }

    pub fn dispatch(&mut self, inst: &Instruction) {
        match *inst {
            Instruction::Inp(a) => {
                let value = self.input.next();
                self.write(a, value)
            }
            Instruction::Add(a, b) => self.write(a, self.read(a) + self.eval(b)),
            Instruction::Mul(a, b) => self.write(a, self.read(a) * self.eval(b)),
            Instruction::Div(a, b) => self.write(a, self.read(a) / self.eval(b)),
            Instruction::Mod(a, b) => self.write(a, self.read(a) % self.eval(b)),
            Instruction::Eql(a, b) => {
                self.write(a, if self.read(a) == self.eval(b) { 1 } else { 0 })
            }
        }
    }

    fn eval(&self, op: Operand) -> i64 {
        match op {
            Operand::Literal(n) => n,
            Operand::Variable(var) => self.read(var),
        }
    }

    fn read(&self, var: Variable) -> i64 {
        match var {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn write(&mut self, var: Variable, value: i64) {
        match var {
            Variable::W => {
                self.w = value;
            }
            Variable::X => {
                self.x = value;
            }
            Variable::Y => {
                self.y = value;
            }
            Variable::Z => {
                self.z = value;
            }
        }
    }
}

impl Port for Vec<i64> {
    fn next(&mut self) -> i64 {
        self.remove(0)
    }
}

fn analyze(instructions: &[Instruction]) {
    let mut last_def: HashMap<Variable, usize> = HashMap::new();

    let deps: Vec<_> = instructions
        .iter()
        .enumerate()
        .flat_map(|(i, inst)| {
            let mut deps = Vec::new();
            for var in inst.read_vars() {
                if let Some(&j) = last_def.get(&var) {
                    deps.push((j, i, var));
                }
            }
            for var in inst.written_vars() {
                last_def.insert(var, i);
            }
            deps
        })
        .collect();

    let mut file = File::create("deps.dot").unwrap();
    writeln!(&mut file, "digraph G {{");
    for (i, inst) in instructions.iter().enumerate() {
        writeln!(&mut file, "{} [shape=\"box\",label=\"{:?}\"];", i, inst);
    }
    for (i, j, var) in deps {
        writeln!(
            &mut file,
            "{} -> {} [label=\"{:?}\", style=\"solid\"];",
            i, j, var
        );
    }
    writeln!(&mut file, "}}");
}

fn validate_alu(instructions: &[Instruction], number: &[i64]) {
    println!("Validate (ALU): {:?}", number);

    let mut model = number.to_vec();
    let mut alu = ALU::new(&mut model);
    alu.execute(&instructions);

    println!("w: {}", alu.w);
    println!("x: {}", alu.x);
    println!("y: {}", alu.y);
    println!("z: {}", alu.z);
}

fn validate_rust(model: &[i64]) {
    println!("Validate (Rust): {:?}", model);

    let mut z = 0;

    let key1 = [1, 1, 1, 26, 26, 1, 26, 26, 1, 1, 26, 1, 26, 26];
    let key2 = [12, 13, 13, -2, -10, 13, -14, -5, 15, 15, -14, 10, -14, -5];
    let key3 = [7, 8, 10, 4, 4, 6, 11, 13, 1, 8, 4, 13, 4, 14];

    for (i, &w) in model.iter().enumerate() {
        /*let z1 = z / key1[i];
        let x2 = z % 26;
        let x3 = x2 + key2[i];
        let y2 = w + key3[i];
        let x4 = if x3 != w { 1 } else { 0 };
        let y3 = x4 * y2;
        let y4 = x4 * 25 + 1;
        let z2 = z1 * y4;
        z = z2 + y3;*/

        /*let z1 = z / key1[i];
        let x2 = z % 26;
        let x3 = x2 + key2[i];
        let y2 = w + key3[i];
        z = if x3 == w { z1 } else { 26 * z1 + y2 };*/

        /*let z1 = z / key1[i];
        let x3 = z % 26 + key2[i];
        z = if x3 == w { z1 } else { 26 * z1 + w + key3[i] };*/

        let y1 = z / key1[i];
        let x3 = z % 26 + key2[i];
        z = if x3 == w { y1 } else { 26 * y1 + w + key3[i] };
    }

    println!("z: {}", z);
}

fn main() {
    let instructions: Vec<Instruction> = io::stdin()
        .lock()
        .lines()
        .map(|s| Instruction::parse(&s.unwrap()))
        .collect();

    analyze(&instructions);
    smt_encode();
    smt_dot();

    // given example
    let ex = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
    validate_alu(&instructions, &ex);
    validate_rust(&ex);

    // first SMT model without optimization
    let ex = vec![3, 3, 1, 9, 1, 9, 1, 5, 7, 9, 3, 2, 1, 3];
    validate_alu(&instructions, &ex);
    validate_rust(&ex);

    // SMT model with maximization
    let ex = vec![7, 9, 1, 9, 7, 9, 1, 9, 9, 9, 3, 9, 8, 5];
    validate_alu(&instructions, &ex);
    validate_rust(&ex);

    // SMT model with minimization
    let ex = vec![1, 3, 1, 9, 1, 9, 1, 3, 5, 7, 1, 2, 1, 1];
    validate_alu(&instructions, &ex);
    validate_rust(&ex);
}

fn smt_encode() {
    let key1 = [1, 1, 1, 26, 26, 1, 26, 26, 1, 1, 26, 1, 26, 26];
    let key2 = [12, 13, 13, -2, -10, 13, -14, -5, 15, 15, -14, 10, -14, -5];
    let key3 = [7, 8, 10, 4, 4, 6, 11, 13, 1, 8, 4, 13, 4, 14];

    let mut file = File::create("monat.smt").unwrap();

    writeln!(&mut file, "(declare-const z0 Int)");
    writeln!(&mut file, "(assert (= 0 z0))");

    for i in 1..=14 {
        writeln!(&mut file, "; Iteration {}", i);

        writeln!(&mut file, "(declare-const w{} Int)", i);
        writeln!(&mut file, "(assert (< 0 w{} 10))", i);

        writeln!(&mut file, "(declare-const x{} Int)", i);
        writeln!(&mut file, "(declare-const y{} Int)", i);
        writeln!(&mut file, "(declare-const z{} Int)", i);

        writeln!(&mut file, "; let y{} = z{} / {}", i, i - 1, key1[i - 1]);
        writeln!(
            &mut file,
            "(assert (= y{} (div z{} {})))",
            i,
            i - 1,
            key1[i - 1]
        );

        writeln!(
            &mut file,
            "; let x{} = z{} % 26 + {}",
            i,
            i - 1,
            key2[i - 1]
        );
        writeln!(
            &mut file,
            "(assert (= x{} (+ (mod z{} 26) {})))",
            i,
            i - 1,
            key2[i - 1]
        );

        writeln!(
            &mut file,
            "; let z{} = if x{} == w{} {{ y{} }} else {{ 26 * y{} + w{} + {} }}",
            i,
            i,
            i,
            i,
            i,
            i,
            key3[i - 1]
        );
        writeln!(
            &mut file,
            "(assert (= z{} (ite (= x{} w{}) y{} (+ (* 26 y{}) w{} {}))))",
            i,
            i,
            i,
            i,
            i,
            i,
            key3[i - 1]
        );
    }

    writeln!(&mut file, "(declare-const model_number Int)");
    let mut model = String::from("w1");
    for i in 2..=14 {
        model = format!("(+ (* {} 10) w{})", model, i);
    }
    writeln!(&mut file, "(assert (= model_number {}))", model);

    // valid model
    writeln!(&mut file, "(assert (= z14 0))");

    // maximize model number
    writeln!(&mut file, "(push)");
    writeln!(&mut file, "(maximize model_number)");
    writeln!(&mut file, "(check-sat)");
    writeln!(&mut file, "(get-value (z14 model_number))");
    writeln!(&mut file, "(pop)");

    // minimize model number
    writeln!(&mut file, "(push)");
    writeln!(&mut file, "(minimize model_number)");
    writeln!(&mut file, "(check-sat)");
    writeln!(&mut file, "(get-value (z14 model_number))");
    writeln!(&mut file, "(pop)");
}

fn smt_dot() {
    let key1 = [1, 1, 1, 26, 26, 1, 26, 26, 1, 1, 26, 1, 26, 26];
    let key2 = [12, 13, 13, -2, -10, 13, -14, -5, 15, 15, -14, 10, -14, -5];
    let key3 = [7, 8, 10, 4, 4, 6, 11, 13, 1, 8, 4, 13, 4, 14];

    let mut file = File::create("deps_smt.dot").unwrap();
    writeln!(&mut file, "digraph G {{");

    for i in 1..=14 {
        writeln!(
            &mut file,
            "\"{}A\" [shape=\"box\",label=\"let y{} = z{} / {}\"];",
            i,
            i,
            i - 1,
            key1[i - 1]
        );
        writeln!(
            &mut file,
            "\"{}B\" [shape=\"box\",label=\"let x{} = z{} % 26 + {}\"];",
            i,
            i,
            i - 1,
            key2[i - 1]
        );
        writeln!(
            &mut file,
            "\"{}C\" [shape=\"box\",label=\"let z{} = if x{} == w{} {{ y{} }} else {{ 26 * y{} + w{} + {} }}\"];",
            i,
            i,
            i,
            i,
            i,
            i,
            i,
            key3[i - 1]
        );

        writeln!(
            &mut file,
            "\"{}A\" -> \"{}C\" [label=\"y\", style=\"solid\"];",
            i, i
        );
        writeln!(
            &mut file,
            "\"{}B\" -> \"{}C\" [label=\"x\", style=\"solid\"];",
            i, i
        );

        if i > 1 {
            writeln!(
                &mut file,
                "\"{}C\" -> \"{}A\" [label=\"z\", style=\"solid\"];",
                i - 1,
                i
            );
            writeln!(
                &mut file,
                "\"{}C\" -> \"{}B\" [label=\"z\", style=\"solid\"];",
                i - 1,
                i
            );
        }
    }

    writeln!(&mut file, "}}");
}
