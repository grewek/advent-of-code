use std::fs;
use std::collections;

#[derive(Debug, Clone, PartialEq)]
enum TokenType<'src> {
    Word(&'src str),
    Number(&'src str),
    Unknown(&'src str),
    Contain,
    Comma,
    Dot,
    Ignored,
    Eol,

}
struct Rule<'src> {
    src: &'src [u8],
    position: usize,
}

impl<'src> Rule<'src> {
    fn new(rule: &str) -> Rule {
        Rule {
            src: rule.as_bytes(),
            position: 0,
        }
    }

    fn scan_identifier(&mut self) -> Option<TokenType<'src>> {
        let start_pos = self.position;
        self.position += 1;

        loop {
            match self.src[self.position] {
                b'a'..=b'z' | b'A'..=b'Z' => self.position += 1,
                _ => {
                    match &self.src[start_pos..self.position] {
                        b"contain" => return Some(TokenType::Contain),
                        b"bag" | b"bags" | b"no" | b"other" => return Some(TokenType::Ignored),
                        _ => return Some(TokenType::Word(std::str::from_utf8(&self.src[start_pos..self.position]).unwrap())),
                    }
                },
            }
        }
    }

    fn consume_space(&mut self) {
        self.position += 1;

        while let b' ' = self.src[self.position] {
            self.position += 1;
        }
    }

    fn scan_number(&mut self) -> Option<TokenType<'src>> {
        let start_pos = self.position;
        self.position += 1;

        loop {
            match self.src[self.position] {
                b'0'..=b'9' => self.position += 1,
                _ => break Some(TokenType::Number(std::str::from_utf8(&self.src[start_pos..self.position]).unwrap())),
            }
        }
    }

    fn scan_comma(&mut self) -> Option<TokenType<'src>> { 
        self.position += 1;
        Some(TokenType::Comma)
    }
    
    fn scan_dot(&mut self) -> Option<TokenType<'src>> {
        self.position += 1;
        Some(TokenType::Dot)
    }
}

impl<'src> Iterator for Rule<'src> {
    type Item = TokenType<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.src.len() {
            return Some(TokenType::Eol);
        }

        if self.src[self.position] == b' ' {
            self.consume_space();
        }

        match self.src[self.position] {
            b'a'..=b'z' | b'A'..=b'Z' => self.scan_identifier(),
            b'1'..=b'9' => self.scan_number(),
            b',' => self.scan_comma(),
            b'.' => self.scan_dot(),
            _ => Some(TokenType::Unknown(std::str::from_utf8(&self.src[self.position..]).unwrap())),
        }
    }
}

fn investigate_bag(bag: &Bag, bag_pool: &collections::HashMap<String, Bag>) -> bool {

    if bag.nodes.is_some() {
        let nodes = bag.nodes.as_ref().unwrap();
        if nodes.contains_key("shiny gold") {
            return true;
        } 

        //TODO: Could we flatten this code to not be recursive ?
        for key in nodes.keys() {
            if investigate_bag(bag_pool.get(key).unwrap(), bag_pool) {
                return true;
            }
        }
    }

    false
}

fn count_bags(current_bag: &Bag, bag_pool: &collections::HashMap<String, Bag>) -> usize {
    let mut count = current_bag.count;
    for (key, amount) in current_bag.nodes.as_ref().unwrap() {
        let next_bag = bag_pool.get(key).unwrap();

        count += amount * count_bags(next_bag, bag_pool);
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut bags_dict: collections::HashMap<String, Bag> = std::collections::HashMap::new();

    for line in input.lines() {
        let mut rule_parser = Parser::new(Rule::new(line));
        rule_parser.parse(&mut bags_dict)
    }

    let part_a = bags_dict.iter().filter(|(_, bag)| investigate_bag(bag, &bags_dict)).count();
    println!("Part 1: Detected {} bags that can hold a shiny gold bag", part_a);
    let shiny_gold = bags_dict.get("shiny gold").unwrap();
    let part_b = count_bags(shiny_gold, &bags_dict);
    println!("Part 2: {} Bags are needed to store one shiny gold bag", part_b);
}

struct Parser<'src> {
    rule: Rule<'src>,
    top_token: Option<TokenType<'src>>
}

impl<'src> Parser<'src> {
    fn new(rule: Rule<'src>) -> Parser<'src> {
        Parser {
            rule,
            top_token: None,
        }
    }

    fn pop_token(&mut self) {
        self.top_token = self.rule.next();
    }

    fn parse_color(&mut self) -> String {
        let mut color_name = String::from("");
        while let Some(TokenType::Word(ident)) = self.top_token {
            color_name.push_str(ident);
            color_name.push(' ');
            self.pop_token();
        }
        color_name.trim().to_string()
    }

    fn parse_number(&mut self) -> usize {
        let result: usize;
        match self.top_token {
            Some(TokenType::Number(num)) => result = num.parse().unwrap(),
            _ => panic!("Number token expected but got {:?}", self.top_token),
        }

        result
    }

    fn parse_bag(&mut self) -> Bag {
        let mut bag = Bag {
            color: String::new(),
            count: 0,

            nodes: None,
        };

        loop {
            match self.top_token {
                Some(TokenType::Word(_)) => bag.color = self.parse_color(),
                Some(TokenType::Number(_)) => bag.count = self.parse_number(),
                _ => break,
            }

            self.pop_token();
        }

        bag
    }

    fn parse(&mut self, bags: &mut collections::HashMap<String, Bag>) {
        self.top_token = self.rule.next();

        let mut main_bag = Bag {
            color: String::new(),
            count: 0,

            nodes: Some(collections::HashMap::new()),
        };

        loop {

            match self.top_token {
                Some(TokenType::Word(_)) => {
                    main_bag = self.parse_bag();
                    main_bag.nodes = Some(collections::HashMap::new());
                }

                Some(TokenType::Number(_)) => {
                    let result = self.parse_bag();
                    main_bag.count += result.count;
                    main_bag.nodes.as_mut().unwrap().insert(result.color, result.count);
                }

                Some(TokenType::Contain) => {
                    self.pop_token();

                    if let Some(TokenType::Number(_)) = self.top_token {
                        let result = self.parse_bag();
                        main_bag.count += result.count;
                        main_bag.nodes.as_mut().unwrap().insert(result.color, result.count);
                    }
                }

                Some(TokenType::Comma) => {
                    self.pop_token();
                    let result = self.parse_bag();
                    main_bag.count += result.count;
                    main_bag.nodes.as_mut().unwrap().insert(result.color, result.count);
                }

                Some(TokenType::Dot) => {
                    self.pop_token();
                }

                Some(TokenType::Eol) => break,
                _ => self.pop_token(),
            }

        }

        bags.insert(main_bag.color.clone(), main_bag);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    color: String,
    count: usize,

    nodes: Option<collections::HashMap<String, usize>>,
}
