use std::num::Wrapping;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write, stdout, stdin};

const TAPE_SIZE: usize = 30000;

enum BrainToken {
    Add,
    Subtract,
    Left,
    Right,
    Read,
    Write,
    OpenLoop(usize),
    CloseLoop(usize),
    None,
}

fn lex(mut r: impl Read) -> Vec<BrainToken> {
    let mut s = String::new();
    r.read_to_string(&mut s)
        .expect("Error reading from source");

    let mut tokens = Vec::new();
    let mut opens = Vec::new();
    let mut index: usize = 0;
    for c in s.chars() {
        match c {
            '+' => tokens.push(BrainToken::Add),
            '-' => tokens.push(BrainToken::Subtract),
            '<' => tokens.push(BrainToken::Left),
            '>' => tokens.push(BrainToken::Right),
            ',' => tokens.push(BrainToken::Read),
            '.' => tokens.push(BrainToken::Write),
            '[' => {
                tokens.push(BrainToken::None);
                opens.push(index);
            }
            ']' => {
                let open = opens.pop()
                    .expect("Mismatched ]");
                tokens[open] = BrainToken::OpenLoop(index);
                tokens.push(BrainToken::CloseLoop(open));
            }
            _ => index -= 1,
        }
        index += 1;
    }
    if !opens.is_empty() {
        panic!("Mismatched [");
    }
    tokens
}

fn run(tokens: Vec<BrainToken>, mut inp: impl Read, mut out: impl Write) {
    let mut next_char = [0; 1];
    let mut tape = [Wrapping(0); TAPE_SIZE];
    let mut ptr = 0;
    let mut pos = 0;

    loop {
        match tokens[pos] {
            BrainToken::Add => tape[ptr] += Wrapping(1),
            BrainToken::Subtract => tape[ptr] -= Wrapping(1),
            BrainToken::Left => ptr -= 1,
            BrainToken::Right => ptr += 1,
            BrainToken::Read => {
                inp.read_exact(&mut next_char)
                    .expect("Error reading next char");
                tape[ptr] = Wrapping(next_char[0]);
            }
            BrainToken::Write => {
                out.write_fmt(format_args!("{}", tape[ptr].0 as char))
                    .expect("Error writing to output");
                out.flush()
                    .expect("Error flushing output");
            }
            BrainToken::OpenLoop(p) => {
                if tape[ptr] == Wrapping(0) {
                    pos = p;
                }
            }
            BrainToken::CloseLoop(p) => {
                if tape[ptr] != Wrapping(0) {
                    pos = p;
                }
            }
            _ => (),
        }
        pos += 1;
        if pos >= tokens.len() {
            break;
        }
    }
}

fn main() {
    let mut args = args();
    let file_name = args.nth(1)
        .expect("No file provided");
    let file = File::open(&file_name)
        .expect("Error opening file");
    
    let tokens = lex(file);
    run(tokens, stdin(), stdout());
}
