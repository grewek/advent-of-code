use std::fs;

enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Direction {
    fn new(cmd: &str) -> Self {
        match cmd.split_once(' ') {
            Some((direction, value)) => Direction::convert(direction, value),
            None => panic!("Empty string cannot be converted into a Direction"),
        }
    }

    fn convert(direction: &str, value: &str) -> Self {
        let distance = value.parse().unwrap();
        match direction.as_bytes() {
            b"forward" => Direction::Forward(distance),
            b"down" => Direction::Down(distance),
            b"up" => Direction::Up(distance),
            _ => panic!("Unknown direction"),
        }
    }
}

struct Position {
    aim: isize,
    x: isize,
    y: isize,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let dirs: Vec<&str> = input.trim().split('\n').collect();

    let mut pos = Position { aim: 0, x: 0, y: 0 };
    for dir in dirs {
        let direction = Direction::new(dir);

        match direction {
            Direction::Forward(p) => pos.x += p as isize,
            Direction::Down(p) => pos.y += p as isize,
            Direction::Up(p) => pos.y -= p as isize,
        }
    }

    println!("Position after the last move: {},{}", pos.x, pos.y);
    println!("Multiplying x pos by y pos: {}", pos.x * pos.y);
}

fn part_two(input: &str) {
    let dirs: Vec<&str> = input.trim().split('\n').collect();

    let mut pos = Position { aim: 0, x: 0, y: 0 };
    for dir in dirs {
        let direction = Direction::new(dir);

        match direction {
            Direction::Forward(p) => {
                pos.x += p as isize;
                pos.y += p as isize * pos.aim;
            }
            Direction::Down(p) => pos.aim += p as isize,
            Direction::Up(p) => pos.aim -= p as isize,
        }
    }

    println!("Position after the last move: {},{}", pos.x, pos.y);
    println!("Multiplying x pos by y pos: {}", pos.x * pos.y);
}
