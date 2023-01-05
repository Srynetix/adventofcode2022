use itertools::Itertools;

const INPUT: &str = include_str!("day01/input.txt");

#[derive(Debug)]
struct ElfCalorieReader {
    calories_per_elf: Vec<u32>,
}

impl ElfCalorieReader {
    pub fn from_str(text: &str) -> Self {
        let calories = text
            .split("\n\n")
            .map(|subl| subl.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
            .collect();

        Self {
            calories_per_elf: calories,
        }
    }

    pub fn max_calories(&self) -> u32 {
        *self.calories_per_elf.iter().max().unwrap()
    }

    pub fn top_three_max_calories(&self) -> u32 {
        self.calories_per_elf.iter().sorted().rev().take(3).sum()
    }
}

pub fn run_part1() -> String {
    ElfCalorieReader::from_str(INPUT).max_calories().to_string()
}

pub fn run_part2() -> String {
    ElfCalorieReader::from_str(INPUT)
        .top_three_max_calories()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::{run_part1, run_part2, ElfCalorieReader};

    const SAMPLE: &str = include_str!("day01/sample.txt");

    #[test]
    fn test_sample() {
        let reader = ElfCalorieReader::from_str(SAMPLE);
        assert_eq!(reader.max_calories(), 24_000);
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(), "66616");
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2(), "199172");
    }
}
