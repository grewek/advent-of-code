use std::collections::HashSet;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    //O(N²)
    let values: Vec<isize> = input
        .lines()
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();

    for (index, num_a) in values.iter().enumerate() {
        for num_b in values[index + 1..].iter() {
            if num_a + num_b == 2020 {
                println!("O(n²)");
                println!("{} + {} = 2020", num_a, num_b);
                println!("- Solution for Day 01 -");
                println!("{} * {} = {}", num_a, num_b, num_a * num_b);
            }
        }
    }

    //O(N)
    let set: HashSet<isize> = input
        .lines()
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();

    for n in &set {
        let searched = 2020 - n;

        if set.contains(&searched) {
            println!("O(N)");
            println!("{} + {} = 2020", n, searched);
            println!("- Solution for Day 01 -");
            println!("{} * {} = {}", n, searched, n * searched);
        }
    }

    //O(NlogN)
    let mut values: Vec<isize> = input
        .lines()
        .map(|v| v.trim().parse::<isize>().unwrap())
        .collect();

    values.sort_unstable();

    for n in &values {
        let searched = 2020 - n;

        if values.binary_search(&searched).is_ok() {
            println!("O(NlogN)");
            println!("{} + {} = 2020", n, searched);
            println!("- Solution for Day 01 -");
            println!("{} * {} = {}", n, searched, n * searched);
            break;
        }
    }
}
