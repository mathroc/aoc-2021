use std::{collections::HashSet, hash::Hash, sync::Arc};

use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq)]
struct Input {
    instructions: Vec<Instruction>,
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Var {
    #[display("w")]
    W,
    #[display("x")]
    X,
    #[display("y")]
    Y,
    #[display("z")]
    Z,
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
enum Val {
    #[display("{0}")]
    Literal(i32),
    #[display("{0}")]
    Var(Var),
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
enum Instruction {
    #[display("inp {0}")]
    Inp(Var),
    #[display("add {0} {1}")]
    Add(Var, Val),
    #[display("mul {0} {1}")]
    Mul(Var, Val),
    #[display("div {0} {1}")]
    Div(Var, Val),
    #[display("mod {0} {1}")]
    Mod(Var, Val),
    #[display("eql {0} {1}")]
    Eql(Var, Val),
}

#[allow(unused_variables)]
#[aoc_generator(day24)]
fn input_generator(input: &str) -> Input {
    Input {
        instructions: input.lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>(),
    }
}

#[test]
fn test_parse_input() {
    assert_eq!(input_generator("inp w
add z w
mod w -12"), Input {
        instructions: vec![
            Instruction::Inp(Var::W),
            Instruction::Add(Var::Z, Val::Var(Var::W)),
            Instruction::Mod(Var::W, Val::Literal(-12)),
        ],
    });
}

impl Val {
    pub fn get(&self, mem: &[i32; 4]) -> i32 {
        match self {
            Val::Var(v) => v.get(mem),
            Val::Literal(val) => *val,
        }
    }
}

impl Var {
    pub fn get(&self, mem: &[i32; 4]) -> i32 {
        match self {
            Var::W => mem[0],
            Var::X => mem[1],
            Var::Y => mem[2],
            Var::Z => mem[3],
        }
    }

    pub fn set(&self, mem: &mut [i32; 4], val: i32) {
        match self {
            Var::W => mem[0] = val,
            Var::X => mem[1] = val,
            Var::Y => mem[2] = val,
            Var::Z => mem[3] = val,
        }
    }
}

impl Instruction {
    pub fn exec(&self, mem: &mut [i32; 4], input: &mut dyn Iterator<Item=&i32>) {
        match self {
            Instruction::Inp(a) => a.set(mem, *input.next().expect("no more input")),
            Instruction::Add(a, b) => {
                let v = a.get(&mem) + b.get(&mem);
                a.set(mem, v);
            },
            Instruction::Mul(a, b) => {
                let v = a.get(&mem) * b.get(&mem);
                a.set(mem, v);
            },
            Instruction::Div(a, b) => {
                let v = a.get(&mem) / b.get(&mem);
                a.set(mem, v);
            },
            Instruction::Mod(a, b) => {
                let v = a.get(&mem) % b.get(&mem);
                a.set(mem, v);
            },
            Instruction::Eql(a, b) => {
                let v = a.get(&mem) == b.get(&mem);
                a.set(mem, v as i32);
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Program {
    next_instruction: usize,
    instructions: Arc<Vec<Instruction>>,
    mem: [i32; 4],
}

impl Program {
    pub fn new(instructions: Arc<Vec<Instruction>>) -> Self {
        Self {
            next_instruction: 0,
            instructions,
            mem: [0; 4],
        }
    }

    pub fn next(&self, input: &mut dyn Iterator<Item=&i32>) -> Program {
        let mut mem = self.mem.clone();

        self.instructions[self.next_instruction].exec(&mut mem, input);

        Program {
            next_instruction: self.next_instruction + 1,
            instructions: self.instructions.clone(),
            mem,
        }
    }

    pub fn advance_unless_input(&self, input: &mut dyn Iterator<Item=&i32>) -> Program {
        match self.instructions.get(self.next_instruction) {
            Some(Instruction::Inp(_)) | None => self.clone(),
            _ => {
                let p = self.next(input);
                
                p.advance_unless_input(input)
            },
        }
    }

    pub fn done(&self) -> bool {
        self.instructions.len() == self.next_instruction
    }

    pub fn optimize(instructions: &mut [Instruction]) {
        for i in 0..instructions.len() - 1 {
            if let Instruction::Inp(v) = instructions[i] {
                match instructions[i + 1] {
                    Instruction::Add(a, _) |
                    Instruction::Mul(a, _) |
                    Instruction::Div(a, _) |
                    Instruction::Mod(a, _) |
                    Instruction::Eql(a, _) |
                    Instruction::Add(_, Val::Var(a)) |
                    Instruction::Mul(_, Val::Var(a)) |
                    Instruction::Div(_, Val::Var(a)) |
                    Instruction::Mod(_, Val::Var(a)) |
                    Instruction::Eql(_, Val::Var(a)) if a == v => {},
                    _ => {
                        instructions.swap(i, i + 1);
                    },
                }
            }
        }
    }
}

#[test]
fn test_optimize() {
    let mut instructions = [
        Instruction::Inp(Var::X),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
    ];

    Program::optimize(&mut instructions);

    assert_eq!(instructions, [
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Inp(Var::X),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
    ]);

    let mut instructions = [
        Instruction::Inp(Var::X),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Inp(Var::X),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
    ];

    Program::optimize(&mut instructions);

    assert_eq!(instructions, [
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Inp(Var::X),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Mul(Var::Y, Val::Literal(0)),
        Instruction::Inp(Var::X),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
    ]);
}

#[test]
fn test_program() {
    let program = Program::new(Arc::new(vec![
        Instruction::Inp(Var::X),
        Instruction::Mul(Var::X, Val::Literal(-1)),
        Instruction::Inp(Var::Y),
        Instruction::Add(Var::X, Val::Var(Var::Y)),
    ]));

    let mut input = [5, 3].iter();

    let program = program.next(&mut input);

    assert_eq!(program.next_instruction, 1);
    assert_eq!(program.mem, [0, 5, 0, 0]);

    let program = program.next(&mut input);

    assert_eq!(program.next_instruction, 2);
    assert_eq!(program.mem, [0, -5, 0, 0]);

    let program = program.next(&mut input);

    assert_eq!(program.next_instruction, 3);
    assert_eq!(program.mem, [0, -5, 3, 0]);

    assert!(!program.done());

    let program = program.next(&mut input);

    assert_eq!(program.next_instruction, 4);
    assert_eq!(program.mem, [0, -2, 3, 0]);

    assert!(program.done());
}

type Output = String;

fn monad(program: Program, s: String, dead_ends: &mut HashSet<Program>, options: &[i32]) -> Option<String> {
    if dead_ends.contains(&program) {
        return None;
    }

    if program.done() {
        return match Var::Z.get(&program.mem) {
            0 => Some(s),
            _ => None,
        };
    }

    let mut empty_input = [].iter().cloned();

    let program = program.advance_unless_input(&mut empty_input);

    for &i in options {
        let p = program.next(&mut [i].iter());
        let p = p.advance_unless_input(&mut empty_input);
        if let Some(res) = monad(p, format!("{}{}", s, i), dead_ends, options) {
            return Some(res);
        }
    }

    dead_ends.insert(program);

    None
}

#[allow(unused_variables)]
#[aoc(day24, part1)]
fn part1(input: &Input) -> Output {
    let mut instructions = input.instructions.clone();

    Program::optimize(&mut instructions);
    
    let program = Program::new(Arc::new(instructions));

    monad(program, "".to_string(), &mut HashSet::new(), &[9, 8, 7, 6, 5, 4, 3, 2, 1]).unwrap()
}

#[allow(unused_variables)]
#[aoc(day24, part2)]
fn part2(input: &Input) -> Output {
    let mut instructions = input.instructions.clone();

    Program::optimize(&mut instructions);
    
    let program = Program::new(Arc::new(instructions));

    monad(program, "".to_string(), &mut HashSet::new(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap()
}