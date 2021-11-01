#![warn(clippy::all, clippy::pedantic)]
use std::fs;

struct Passport<'a>([Option<&'a str>; 7]);

impl<'a> Passport<'a>
{
    fn new_passport() -> Passport<'a>
    {
        Passport([None; 7])
    }

    fn is_valid(&self) -> bool
    {
        self.0.iter().all(|v| v.is_some()) 
    }

    fn data_is_valid(&self) -> bool
    {
        if self.check_byr() &&
            self.check_iyr() && self.check_eyr() && self.check_hgt() &&
            self.check_hcl() && self.check_ecl() && self.check_pid()
        {
            return true;
        }

        false
    }

    fn check_byr(&self) -> bool
    {
        let byr: usize = self.0[0].unwrap().parse().unwrap();

        byr >= 1920 && byr <= 2002
    }

    fn check_iyr(&self) -> bool
    {
        let iyr: usize = self.0[1].unwrap().parse().unwrap();

        iyr >= 2010 && iyr <= 2020
    }

    fn check_eyr(&self) -> bool
    {
        let eyr: usize = self.0[2].unwrap().parse().unwrap();

        eyr >= 2020 && eyr <= 2030
    }

    fn check_hgt(&self) -> bool
    {
        let height_raw: &str = self.0[3].unwrap();
        let height = &height_raw[0..height_raw.len() - 2];
        let unit_type = &height_raw[height_raw.len() - 2..];

        match unit_type
        {
            "cm" => {
                let height: usize = height.parse().unwrap();
                height >= 150 && height <= 193
            },
            "in" => {
                let height: usize = height.parse().unwrap();
                height >= 59 && height <= 76
            },
            _ => false,
        }
    }

    fn check_hcl(&self) -> bool
    {
        let hair_color = self.0[4].unwrap();
        if let Some(hex_value) = hair_color.strip_prefix('#') {

            let value = usize::from_str_radix(hex_value, 16);
            
            if value.is_ok() && hex_value.len() == 6 {
                return true;
            }
        }

        false
    }

    fn check_ecl(&self) -> bool
    {
        match self.0[5].unwrap()
        {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }

    fn check_pid(&self) -> bool
    {
        self.0[6].unwrap().len() == 9
    }

    fn insert_data(&mut self, key: &str, value: &'a str) {
        match key
        {
            "byr" => self.0[0] = Some(value),
            "iyr" => self.0[1] = Some(value),
            "eyr" => self.0[2] = Some(value),
            "hgt" => self.0[3] = Some(value),
            "hcl" => self.0[4] = Some(value),
            "ecl" => self.0[5] = Some(value),
            "pid" => self.0[6] = Some(value),
            _ => (),
        };

    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut passport = Passport::new_passport();
    let mut valid_passport_count_part_a = 0;
    let mut valid_passport_count_part_b = 0;

    for line in input.split("\n\n")
    {
        for (key, value) in line.split_whitespace().map(|l| l.split_once(':').unwrap()) {
            passport.insert_data(key, value);
        }

        if passport.is_valid()
        {
            valid_passport_count_part_a += 1;

            if passport.data_is_valid()
            {
                valid_passport_count_part_b += 1;
            }
        }
        
        passport = Passport::new_passport();
    }

    println!("valid passports for part a: {}", valid_passport_count_part_a);
    println!("valid passports for part b: {}", valid_passport_count_part_b);
}
