use std::fs;

struct AnswerSheet {
    answers: u32,
}

impl AnswerSheet {
    fn new(raw: &str) -> AnswerSheet {
        let mut answers = 0;
        for letter in raw.as_bytes() {
            let index = 25 - (b'z' - letter);
            answers |= 1 << index;
        }

        AnswerSheet {
            answers,
        }
    }

    fn intersect(&self, other: &AnswerSheet) -> AnswerSheet {
        AnswerSheet {
            answers: self.answers & other.answers,
        }
    }
}

struct Group {
    sheets: Vec<AnswerSheet>,
}

impl Group {
    fn new(group: &str) -> Self {
        let mut sheets = vec![];
        for line in group.split_whitespace() {
            sheets.push(AnswerSheet::new(line.trim()))
        }

        Group {
            sheets,
        }
    }

    fn group_intersection(&self) -> AnswerSheet {
        self.sheets.iter().fold(AnswerSheet { answers: u32::MAX, }, |acc, sheet| acc.intersect(sheet))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;
    for group in input.split("\n\n") {
        let g = Group::new(group);
        result += g.group_intersection().answers.count_ones();
    }

    println!("{}", result);
}
