use std::{
    collections::HashMap,
    fmt,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
};

#[derive(Clone, Debug, Eq)]
struct Chemical(i64, String);

impl Chemical {
    pub fn parse(s: &str) -> Chemical {
        let parts: Vec<&str> = s.trim().split(" ").collect();
        Chemical(
            parts[0].parse().expect("failed to parse chemical quantity"),
            parts[1].to_owned(),
        )
    }

    pub fn name(&self) -> &str {
        &self.1
    }

    pub fn quantity(&self) -> i64 {
        self.0
    }
}

impl Hash for Chemical {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // don't hash quantity
        self.1.hash(state);
    }
}

impl PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity(), self.name())
    }
}

fn read_formulas(input: &str) -> HashMap<String, (i64, Vec<Chemical>)> {
    //let formulas: Vec<Formula> = input.lines().map(|line| Formula::parse(line)).collect();
    let mut formulas = HashMap::new();
    for line in input.lines() {
        // split on '=>'
        let parts: Vec<&str> = line.split("=>").map(|p| p.trim()).collect();
        let chemical = Chemical::parse(parts[1]);
        formulas.insert(
            chemical.name().to_owned(),
            (
                chemical.quantity(),
                parts[0].split(",").map(|c| Chemical::parse(c)).collect(),
            ),
        );
    }

    formulas
}

fn part1(formulas: &HashMap<String, (i64, Vec<Chemical>)>, n: i64) -> i64 {
    // calculate needed totals backwards, starting at fuel
    let mut reqs: HashMap<&str, i64> = HashMap::new();
    let mut ingredients = vec!["FUEL"];
    reqs.insert("FUEL", n);

    while !ingredients.is_empty() {
        let chem = ingredients.remove(0);
        if chem != "ORE" && reqs[chem] > 0 {
            let r = reqs[chem]; // how many we required for a given chemical
            let f = &formulas[chem]; // the formula for how to build the chemical
            let amt = ((r - 1) / f.0) + 1; // how many times we need to run the formula to get the number we need
            reqs.insert(chem, r - (f.0 * amt)); // update our reguirements with the amount the formula we'll build

            // for each ingrdient in our formula, add the quantity it builds times the amount we'll
            // need as computed above
            for n in &f.1 {
                let c = reqs.get(n.name()).map(|i| *i).unwrap_or(0);
                reqs.insert(n.name(), c + (n.quantity() * amt));
                ingredients.push(n.name());
            }

            ingredients.push(chem);
        }
    }

    reqs["ORE"]
}

fn part2(formulas: &HashMap<String, (i64, Vec<Chemical>)>) -> i64 {
    let goal: i64 = 1000000000000;
    let mut base = 0;
    let mut size = goal;
    let mut ore = 0;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        ore = part1(formulas, mid);
        if ore <= goal {
            base = mid;
        }
        size -= half;
    }

    if ore == goal {
        base
    } else {
        base + ((ore < goal) as i64) - 1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the file
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    let formulas = read_formulas(&input);

    let ore = part1(&formulas, 1);
    println!("Minimum Ore:   {}", ore);
    println!("Maxiumum Fuel: {}", part2(&formulas));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_01() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_1.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let ore = part1(&formulas, 1);
        assert_eq!(ore, 31);
        Ok(())
    }

    #[test]
    fn part1_02() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_2.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let ore = part1(&formulas, 1);
        assert_eq!(ore, 165);
        Ok(())
    }

    #[test]
    fn part1_03() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_3.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let ore = part1(&formulas, 1);
        assert_eq!(ore, 13312);
        Ok(())
    }

    #[test]
    fn part1_04() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_4.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let ore = part1(&formulas, 1);
        assert_eq!(ore, 180697);
        Ok(())
    }

    #[test]
    fn part1_05() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_5.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let ore = part1(&formulas, 1);
        assert_eq!(ore, 2210736);
        Ok(())
    }

    #[test]
    fn part2_01() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_3.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let fuel = part2(&formulas);
        assert_eq!(fuel, 82892753);
        Ok(())
    }

    #[test]
    fn part2_02() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_4.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let fuel = part2(&formulas);
        assert_eq!(fuel, 5586022);
        Ok(())
    }

    #[test]
    fn part2_03() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("input_test_5.txt")?;
        let mut input = String::new();
        f.read_to_string(&mut input)?;

        let formulas = read_formulas(&input);
        let fuel = part2(&formulas);
        assert_eq!(fuel, 460664);
        Ok(())
    }
}
