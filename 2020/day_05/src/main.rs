use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut tickets: Vec<usize> = input.lines().map(|l| ticket_to_seat(l.trim())).collect();
    tickets.sort();

    println!("Highest ticket: {:?}", tickets.last());
    for chunk in tickets.chunks(2) {
        if chunk[0] + 1 != chunk[1] {
            println!("Found seat: {}", chunk[0] + 1);
        }
    }

}

fn ticket_to_seat(input: &str) -> usize
{
    let row= &input[0..7];
    let column = &input[7..];

    let result_row = process_command(Target::Row, row);
    let result_column = process_command(Target::Column, column);
    result_row * 8 + result_column
}

enum Partition {
    High,
    Low,
}

fn partition(min: usize, max: usize, command: Partition) -> (usize, usize)
{
    match command
    {
        Partition::High => (min, ((min + max) / 2)),
        Partition::Low  => ((min + max + 1) / 2, max),
    }
}

enum Target {
    Row,
    Column,
}

fn process_command(find: Target, cmd_repr: &str) -> usize {
    let mut current = match find {
        Target::Row => (0, 127),
        Target::Column => (0, 7),
    };

    for letter in cmd_repr.chars() {
        match letter {
            'f' | 'F' => current = partition(current.0, current.1, Partition::High),
            'b' | 'B' => current = partition(current.0, current.1, Partition::Low),
            'r' | 'R' => current = partition(current.0, current.1, Partition::Low),
            'l' | 'L' => current = partition(current.0, current.1, Partition::High),
            _ => panic!("Unknown Command letter ! {}", letter),
        };
 
        if current.0 == current.1 {
            return current.0
        }
    }

    0
}
