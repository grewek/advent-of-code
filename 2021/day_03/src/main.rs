use std::fs;

#[derive(Debug)]
struct Values<'b> {
    vals: Vec<&'b str>, 
    length: usize,
}

enum MostCommon {
    Zero,
    One,
    Equal,
}

impl<'b> Values<'b> {
    fn new(vals: Vec<&'b str>, length: usize) -> Self {
        Values {
            vals,
            length,
        }
    }

    fn is_bit_set(&self, row: usize, bit_pos: usize) -> bool {
        let bit = self.vals[row].chars().nth(bit_pos).unwrap();
        match bit {
            '0' => false,
            '1' => true,
            _ => false,
        }
    }

    fn check_most_common(&self, bit_pos: usize) -> MostCommon {
        let mut zero_count = 0;
        let mut one_count = 0;
        for index in 0..self.vals.len() {
            match self.is_bit_set(index, bit_pos) {
                false => zero_count += 1,
                true => one_count += 1,
            }
        }

        if zero_count == one_count {
            return MostCommon::Equal;
        }

        if zero_count > one_count {
            return MostCommon::Zero;
        }

        MostCommon::One
    }


    fn search(&self) {
        for index in 0..self.vals.len() {

        }
    }
    fn check_all_values(&self, bit_pos: usize) -> (usize, usize) {
        let mut zero_count = 0;
        let mut one_count = 0;
        for index in 0..self.vals.len() {
            match self.is_bit_set(index, bit_pos) {
                false => zero_count += 1,
                true => one_count += 1,
            }
        }

        (zero_count, one_count)
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();
    let length = values[0].len();

    let bits = Values::new(values, length);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for pos in 0..bits.length {
        let result = bits.check_all_values(pos);
        let most_common = usize::max(result.0, result.1);
        let least_common = usize::min(result.0, result.1);

        if most_common == result.0 {
            //TODO: We have zero as the most common value
            gamma.push('0');
            epsilon.push('1');
        } else {
            //TODO: The most common value was one
            gamma.push('1');
            epsilon.push('0');
        }
    }


    let g_value: isize = isize::from_str_radix(&gamma, 2).unwrap();
    let e_value: isize = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("The power consumption is {}", g_value * e_value);
}

