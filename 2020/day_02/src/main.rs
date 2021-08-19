use std::collections::HashMap;

#[derive(Debug)]
struct PasswordRule {
    count_min: usize,
    count_max: usize,
    letter: char,
    password: String,
}

impl From<&str> for PasswordRule {
    fn from(input: &str) -> Self {
        let mut rule = PasswordRule::new();
        for (rule_index, raw_rule) in input.split_whitespace().enumerate() {
            rule = match rule_index {
                0 => rule.parse_pw_lengths(raw_rule),
                1 => rule.parse_pw_character(raw_rule),
                2 => rule.parse_password(raw_rule),
                _ => panic!("Error: Invalid password part: {} {}", rule_index, raw_rule),
            };
        }

        println!("{:?}", rule);
        rule
    }
}

impl PasswordRule {
    fn new() -> Self {
        Self {
            count_min: 0,
            count_max: 0,
            letter: ' ',
            password: "".to_string(),
        }
    }
    fn parse_pw_lengths(self, input: &str) -> Self {
        let mut lengths = input.split('-');

        Self {
            count_min: lengths.next().unwrap().parse::<usize>().unwrap(),
            count_max: lengths.next().unwrap().parse::<usize>().unwrap(),
            letter: self.letter,
            password: self.password,
        }
    }

    fn parse_pw_character(self, input: &str) -> Self {
        Self {
            count_min: self.count_min,
            count_max: self.count_max,
            letter: input.chars().next().unwrap(),
            password: self.password,
        }
    }

    fn parse_password(self, input: &str) -> Self {
        Self {
            count_min: self.count_min,
            count_max: self.count_max,
            letter: self.letter,
            password: input.to_string(),
        }
    }

    fn is_valid_part_a(&self) -> bool {
        let mut letters = HashMap::new();
        letters.entry(self.letter).or_insert(0);

        for c in self.password.chars() {
            *letters.entry(c).or_insert(0) += 1
        }

        let lc = *letters.get(&self.letter).unwrap();

        if lc >= self.count_min && lc <= self.count_max {
            return true;
        }

        false
    }

    fn is_valid_part_b(&self) -> bool {
        let chars = self.password.as_bytes();
        let pos_a = chars.get(self.count_min - 1);
        let pos_b = chars.get(self.count_max - 1);

        let byte_value = self.letter as u8;

        match (pos_a, pos_b) {
            (Some(a), Some(b)) => {
                *a == byte_value && *b != byte_value || *a != byte_value && *b == byte_value
            }
            (Some(a), None) => *a == byte_value,
            (None, Some(b)) => *b == byte_value,
            (None, None) => false,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", input);

    let password_rules: Vec<PasswordRule> = input.lines().map(|line| line.into()).collect();
    let valid_passwords = password_rules
        .iter()
        .filter(|rule| rule.is_valid_part_a())
        .count();
    println!("--- Result for Day_02 Part A ---");
    println!("Valid Passwords: {}", valid_passwords);

    let valid_passwords = password_rules
        .iter()
        .filter(|rule| rule.is_valid_part_b())
        .count();
    println!("--- Result for Day_02 Part B ---");
    println!("Valid Passwords: {}", valid_passwords);
}
