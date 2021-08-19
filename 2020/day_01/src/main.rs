use std::collections::HashSet;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    day_01_part_a(&input);
    day_01_part_b(&input)
}

fn day_01_part_a(input: &str) {
    //O(N)
    let set: HashSet<isize> = input
        .lines()
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();

    for n in &set {
        let searched = 2020 - n;

        if set.contains(&searched) {
            println!("--- Solution for Day 01 Part A ---");
            println!("O(N)");
            println!("{} + {} = 2020", n, searched);
            println!("{} * {} = {}", n, searched, n * searched);
            break;
        }
    }
}

fn day_01_part_b(input: &str) {
    //O(N²)
    let mut values: Vec<isize> = input
        .lines()
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();

    values.sort_unstable();

    for (index_i, i) in values.iter().enumerate() {
        for j in values[index_i + 1..].iter() {
            let searched = 2020 - i - j;
            if values.binary_search(&searched).is_ok() {
                println!("--- Solution for Day 01 Part B ---");
                println!("O(N²)");
                println!("{} + {} + {} = 2020", i, j, searched);
                println!("{} * {} * {} = {}", i, j, searched, i * j * searched);
            }
        }
    }
}
