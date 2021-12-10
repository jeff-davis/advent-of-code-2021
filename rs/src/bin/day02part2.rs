
use std::io;
use std::io::BufRead;

#[derive(Debug,PartialEq)]
enum Instruction {
    Up(u64),
    Down(u64),
    Forward(u64),
}

fn parse_instr(s: &str) -> Instruction {
    let (kind_str, val_str) = s.split_once(' ').unwrap();
    let val = val_str.trim().parse::<u64>().unwrap();
    match kind_str {
        "up" => Instruction::Up(val),
        "down" => Instruction::Down(val),
        "forward" => Instruction::Forward(val),
        _ => panic!("unexpected instruction kind: {}", kind_str),
    }
}

fn exec_instr<T: Iterator<Item = Instruction>>(instructions: T) -> (u64, u64) {
    let mut h_pos: u64 = 0;
    let mut depth: u64 = 0;
    let mut aim: u64 = 0;
    for instr in instructions {
        match instr {
            Instruction::Up(v) => aim -= v,
            Instruction::Down(v) => aim += v,
            Instruction::Forward(v) => {
                h_pos += v;
                depth += (aim * v) as u64;
            },
        }
    }

    (h_pos, depth)
}

fn main() {
    let stdin = io::stdin();
    let input_instr = stdin.lock()
        .lines()
        .map(|l| parse_instr(&l.unwrap()));

    let (h_pos, depth) = exec_instr(input_instr);
    println!("depth = {}, horizontal position = {}, product = {}",
             depth, h_pos, depth*h_pos);
}

#[test]
fn test_parse() {
    assert_eq!(parse_instr("up 1000"), Instruction::Up(1000));
    assert_eq!(parse_instr("down 0"), Instruction::Down(0));
    assert_eq!(parse_instr("forward 3"), Instruction::Forward(3));
}

#[test]
fn test_parse_trim() {
    assert_eq!(parse_instr("up  1000"), Instruction::Up(1000));
    assert_eq!(parse_instr("forward \t30 "), Instruction::Forward(30));
}

#[test]
#[should_panic]
fn test_parse_panic_no_whitespace() {
    assert_eq!(parse_instr("up1000"), Instruction::Up(1000));
}

#[test]
#[should_panic]
fn test_parse_panic_leading_whitespace() {
    assert_eq!(parse_instr(" down 1000"), Instruction::Up(1000));
}

#[test]
#[should_panic]
fn test_parse_panic_tab() {
    assert_eq!(parse_instr("up\t1000"), Instruction::Up(1000));
}

#[test]
#[should_panic]
fn test_parse_panic_neg() {
    assert_eq!(parse_instr("forward -1000"), Instruction::Up(1000));
}

#[test]
fn test_exec_empty() {
    let empty: Vec<Instruction> = vec![];
    assert_eq!(exec_instr(empty.into_iter()), (0, 0));
}

#[test]
fn test_exec() {
    let instructions = vec![
        Instruction::Down(3),
        Instruction::Forward(100),
        Instruction::Down(2),
        Instruction::Forward(3),
        Instruction::Up(1),
    ];
    assert_eq!(exec_instr(instructions.into_iter()), (103, 315));
}

#[test]
fn test_exec_example() {
    let instructions = vec![
        Instruction::Forward(5),
        Instruction::Down(5),
        Instruction::Forward(8),
        Instruction::Up(3),
        Instruction::Down(8),
        Instruction::Forward(2),
    ];
    assert_eq!(exec_instr(instructions.into_iter()), (15, 60));
}

#[test]
#[should_panic]
fn test_exec_panic_neg() {
    let instruction_set2 = vec![
        Instruction::Down(3),
        Instruction::Forward(100),
        Instruction::Down(2),
        Instruction::Forward(3),
        Instruction::Up(7),
    ];
    assert_eq!(exec_instr(instruction_set2.into_iter()), (0, 0));
}
