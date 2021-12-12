use std::fs;


fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap()).collect::<Vec<usize>>();

    println!("part_a: {}, part_b: {}", part_one(&input), part_two(&input));
}

fn part_one(input: &[usize]) -> usize {
    let mut count = 0;
    let mut top = input[0];

    for value in input.iter().skip(1) {
        if *value > top {
            count += 1;
        }

        top = *value
    }

    count
}

fn part_two(input: &[usize]) -> usize {
    let start = input[0] + input[1] + input[2];
    let result = input.windows(3).skip(1).fold((0 as usize, start), |acc, win| {
        let next = win[0] + win[1] + win[2];

        if next > acc.1 {
            return (acc.0 + 1, next);
        }

        (acc.0, next)
    });

    result.0
}
