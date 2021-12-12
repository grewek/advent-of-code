use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

struct Parser<'src> {
    src: &'src [u8],
    position: usize,
}

impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser {
            src: input.as_bytes(),
            position: 0,
        }
    }
    fn scan_operation(&mut self) -> Option<Operation> {
        let start = self.position;
        while let b'a'..=b'z' | b'A'..=b'Z' = self.src[self.position] {
            self.position += 1;
        }

        match &self.src[start..self.position] {
            b"acc" => Some(Operation::Acc(self.scan_number())),
            b"jmp" => Some(Operation::Jmp(self.scan_number())),
            b"nop" => Some(Operation::Nop(self.scan_number())),
            _ => panic!("Unknown keyword!"),
        }
    }

    fn scan_number(&mut self) -> isize {
        while let b' ' | b'+' = self.src[self.position] {
            self.position += 1;
        }

        let start = self.position;
        while let b'0'..=b'9' | b'-' = self.src[self.position] {
            self.position += 1;
        }

        std::str::from_utf8(&self.src[start..self.position]).unwrap().parse().unwrap()
    }
}

impl Iterator for Parser<'_> {
   type Item = Operation;

   fn next(&mut self) -> Option<Self::Item> {
       if self.position >= self.src.len() - 1 {
           return None;
       }

       loop {
           let next_char = self.src[self.position];

           match next_char {
               b'a'..=b'z' | b'A'..=b'Z' => {
                   return self.scan_operation();
               }
               b'\r' | b'\n' => {
                   self.position += 1;
               }
            
               _ => panic!("Lexer has reached a unknown symbol in the character stream {}", next_char),
           };
       }
   }
}

#[derive(Debug)]
struct VirtualMachine<'prog> {
    pc: usize,
    accumulator: isize,
    prog: &'prog [Operation],
}

impl<'prog> VirtualMachine<'prog> {
    fn new(prog: &'prog [Operation]) -> Self {
        Self {
            pc: 0,
            accumulator: 0,
            prog,
        }
    }

    fn step(&mut self) {
        if self.pc >= self.prog.len() {
            return;
        }

        let instruction = &self.prog[self.pc];


        match instruction {
            Operation::Acc(num) => {
                self.accumulator += num;
                self.pc += 1;
            }
            Operation::Jmp(num) => self.pc = ((self.pc as isize) + num) as usize,
            Operation::Nop(_) => self.pc += 1,
        }
    }
}

fn patch(program: &mut [Operation], position: usize) -> bool {
    match program[position] {
        Operation::Acc(_) => false,
        Operation::Jmp(num) => {
            program[position] = Operation::Nop(num);
            return true;
        }
        Operation::Nop(num) => {
            program[position] = Operation::Jmp(num);
            return true;
        }
    }
}

fn run(prog: &[Operation]) {
    let mut vm = VirtualMachine::new(prog);
    let mut counter_set = HashSet::new();
    loop {
        if vm.pc >= prog.len() {
            break;
        }
        vm.step();

        if counter_set.contains(&vm.pc) {
            println!("First repeatedly executed instruction was: {} and the accumulator value is: {}", vm.pc, vm.accumulator);
            return;
        } else {
            counter_set.insert(vm.pc);
        }
    }

    println!("Reached the end of the program with accumulator value: {}", vm.accumulator);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut program: Vec<Operation> = Parser::new(&input).collect();

    //NOTE: Bruteforce...
    for idx in 0..program.len() - 1 {
        if patch(&mut program, idx) {
            run(&program);
            patch(&mut program, idx);
        }
    }
}
