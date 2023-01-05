use std::collections::HashSet;

const INPUT: &str = include_str!("day03/input.txt");

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Item(char);

impl Item {
    pub fn new(c: char) -> Self {
        Self(c)
    }

    pub fn priority(&self) -> u32 {
        match self.0 {
            c @ 'a'..='z' => (c as u32 - 'a' as u32) + 1,
            c @ 'A'..='Z' => (c as u32 - 'A' as u32) + 27,
            c => panic!("unsupported char {c}"),
        }
    }
}

#[derive(Debug)]
struct Compartment {
    items: HashSet<Item>,
}

impl Compartment {
    pub fn from_str(text: &str) -> Self {
        Self {
            items: text.chars().map(Item::new).collect(),
        }
    }

    pub fn intersection(&self, other: &Self) -> HashSet<Item> {
        self.items.intersection(&other.items).copied().collect()
    }

    pub fn items(&self) -> &HashSet<Item> {
        &self.items
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: [Compartment; 2],
}

impl Rucksack {
    pub fn from_str(text: &str) -> Self {
        let (p1, p2) = text.split_at(text.len() / 2);
        Self {
            compartments: [Compartment::from_str(p1), Compartment::from_str(p2)],
        }
    }

    pub fn only_common_item(&self) -> Item {
        let [compartment1, compartment2] = &self.compartments;
        let common_items = compartment1.intersection(compartment2);

        assert!(
            common_items.len() == 1,
            "there should be only one common item in a rucksack"
        );
        *common_items.iter().next().unwrap()
    }

    pub fn total_items(&self) -> HashSet<Item> {
        self.compartments
            .iter()
            .flat_map(|x| x.items())
            .copied()
            .collect::<HashSet<_>>()
    }
}

#[derive(Debug)]
struct RucksackParser {
    rucksacks: Vec<Rucksack>,
}

impl RucksackParser {
    pub fn from_str(text: &str) -> Self {
        let rucksacks = text.lines().map(Rucksack::from_str).collect();
        Self { rucksacks }
    }

    pub fn priority_sum(&self) -> u32 {
        self.rucksacks
            .iter()
            .map(|r| r.only_common_item().priority())
            .sum()
    }

    fn group_only_common_item(&self, group: &[Rucksack]) -> Item {
        let [g1, g2, g3] = group else {
            panic!("chunk should be of size 3");
        };

        let items = g1
            .total_items()
            .intersection(&g2.total_items())
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&g3.total_items())
            .copied()
            .collect::<HashSet<_>>();
        assert!(
            items.len() == 1,
            "there should be only one common item in a group"
        );

        *items.iter().next().unwrap()
    }

    pub fn group_priority_sum(&self) -> u32 {
        self.rucksacks
            .chunks_exact(3)
            .map(|group| self.group_only_common_item(group).priority())
            .sum()
    }
}

pub fn run_part1() -> String {
    RucksackParser::from_str(INPUT).priority_sum().to_string()
}

pub fn run_part2() -> String {
    RucksackParser::from_str(INPUT)
        .group_priority_sum()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::{run_part1, run_part2, RucksackParser};

    const SAMPLE: &str = include_str!("day03/sample.txt");

    #[test]
    fn test_sample() {
        assert_eq!(RucksackParser::from_str(SAMPLE).priority_sum(), 157);
        assert_eq!(RucksackParser::from_str(SAMPLE).group_priority_sum(), 70);
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(), "7553");
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2(), "2758");
    }
}
