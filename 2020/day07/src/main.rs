use regex::Regex;
use std::{
    cmp::PartialEq,
    collections::{HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
    io,
};

#[derive(Clone, Debug, Eq)]
struct Bag(u32, String);

impl Bag {
    pub fn named(name: impl Into<String>) -> Self {
        Bag(1, name.into())
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

type Rules = HashMap<String, HashSet<Bag>>;

//#[derive(Debug)]
//struct Rule(String, HashSet<Bag>);

fn read_rules(input: &str) -> Rules {
    let re = Regex::new(
        r"(?P<source>\w+ \w+) bags? contain (?P<dests>(?:(?:\d+ \w+ \w+) bags?(?:, )?)+|(?:no other bags))",
    )
    .unwrap();

    let dest_re = Regex::new(r"(?P<count>\d+) (?P<bag>\w+ \w+)").unwrap();

    let mut rules = HashMap::new();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let source = caps.name("source").unwrap().as_str().to_owned();
            let dests = match caps.name("dests") {
                Some(dests) => dests
                    .as_str()
                    .split(", ")
                    .filter_map(|s| dest_re.captures(s))
                    .map(|cap| {
                        let num: u32 = cap.name("count").unwrap().as_str().parse().unwrap();
                        let bag = cap.name("bag").unwrap().as_str().to_owned();
                        Bag(num, bag)
                    })
                    .collect::<HashSet<_>>(),
                None => HashSet::new(),
            };

            rules.insert(source, dests);
        }
    }

    rules
}

fn part01(rules: &Rules) -> usize {
    let mut answer = HashSet::new();
    let mut stack = vec!["shiny gold".to_owned()];

    while let Some(bag) = stack.pop() {
        let mut sources = rules
            .iter()
            .filter(|(_, r)| r.contains(&Bag::named(bag.clone())))
            .map(|(s, _)| s.clone())
            .collect::<Vec<_>>();

        for bag in &sources {
            answer.insert(bag.clone());
        }
        stack.append(&mut sources);
    }

    answer.len()
}

fn part02(rules: &Rules, bag: Bag) -> u32 {
    if let Some(inside) = rules.get(&bag.1) {
        let count;
        if inside.is_empty() {
            count = bag.0;
        } else {
            let i = inside.iter().map(|b| part02(rules, b.clone())).sum::<u32>();
            count = bag.0 * i + bag.0;
        }
        count
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let rules = read_rules(&input);

    // Part 1: 164
    println!("Part 1: {}", part01(&rules));

    // Part 2: 7872
    println!("Part 2: {}", part02(&rules, Bag::named("shiny gold")) - 1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules() -> Rules {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        read_rules(&input)
    }

    #[test]
    fn aoc_1() {
        let rules = rules();
        let output = part01(&rules);
        let expected = 4;
        assert_eq!(output, expected, "did not match");
    }

    #[test]
    fn aoc_2() {
        let rules = rules();
        let output = part02(&rules, Bag::named("shiny gold")) - 1;
        let expected = 32;
        assert_eq!(output, expected, "did not match");
    }

    #[test]
    fn aoc_3() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let rules = read_rules(&input);
        let output = part02(&rules, Bag::named("shiny gold")) - 1;
        let expected = 126;
        assert_eq!(output, expected, "did not match");
    }
}
