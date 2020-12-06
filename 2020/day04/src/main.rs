use std::{fs, io, str::FromStr};

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
struct Passport {
    // birth year
    byr: Option<u16>,

    // issue year
    iyr: Option<u16>,

    // expiration year
    eyr: Option<u16>,

    // height
    hgt: Option<String>,

    // hair color
    hcl: Option<String>,

    // eye color
    ecl: Option<EyeColor>,

    // passport id
    pid: Option<String>,

    // country id
    cid: Option<String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if !(self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some())
        {
            return false;
        }

        let mut valid = true;
        let byr = self.byr.unwrap();
        valid = valid && byr >= 1920 && byr <= 2002;

        let iyr = self.iyr.unwrap();
        valid = valid && iyr >= 2010 && iyr <= 2020;

        let eyr = self.eyr.unwrap();
        valid = valid && eyr >= 2020 && eyr <= 2030;

        // height
        let hgt = self.hgt.as_ref().unwrap();
        let (hgt, ty) = hgt.split_at(hgt.len() - 2);
        valid = valid
            && match ty {
                "in" => match hgt.parse::<u8>() {
                    Ok(h) if h >= 59 && h <= 76 => true,
                    _ => false,
                },
                "cm" => match hgt.parse::<u8>() {
                    Ok(h) if h >= 150 && h <= 193 => true,
                    _ => false,
                },
                _ => false,
            };

        // hair color
        let (pound, hcl) = self.hcl.as_ref().unwrap().split_at(1);
        if pound != "#" || hcl.len() != 6 {
            valid = false;
        } else {
            valid = valid && hcl.chars().all(|c| c.is_ascii_hexdigit());
        }

        // eye color validated on parse...skipping here

        // passport id
        let pid = self.pid.as_ref().unwrap();
        valid = valid && pid.len() == 9;
        valid = valid && pid.parse::<u64>().is_ok();

        // cid is optional, ignore

        valid
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut passports = vec![];
    for line in input.split("\n\n") {
        let mut passport = Passport::default();
        for section in line.split_whitespace() {
            let (field, value) = section.split_at(4);
            match field {
                "byr:" => passport.byr = value.parse().ok(),
                "iyr:" => passport.iyr = value.parse().ok(),
                "eyr:" => passport.eyr = value.parse().ok(),
                "hgt:" => passport.hgt = Some(value.to_owned()),
                "hcl:" => passport.hcl = Some(value.to_owned()),
                "ecl:" => passport.ecl = value.parse().ok(),
                "pid:" => passport.pid = Some(value.to_owned()),
                "cid:" => passport.cid = Some(value.to_owned()),
                _ => (),
            }
        }
        passports.push(passport);
    }

    // part 1 answer: 264
    // part 2 answer: 224

    let valid = passports.iter().filter(|p| p.is_valid()).count();
    println!("{} valid passports", valid);

    Ok(())
}
