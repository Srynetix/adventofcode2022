use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day04/input.txt");

#[derive(Debug, Eq, PartialEq)]
struct AssignmentPair {
    range_a: RangeInclusive<usize>,
    range_b: RangeInclusive<usize>,
}

impl AssignmentPair {
    pub fn from_str(s: &str) -> Self {
        fn parse_range(s: &str) -> RangeInclusive<usize> {
            let s = s.split('-').collect::<Vec<_>>();
            RangeInclusive::new(s[0].parse().unwrap(), s[1].parse().unwrap())
        }

        let s = s.split(',').collect::<Vec<_>>();
        Self {
            range_a: parse_range(s[0]),
            range_b: parse_range(s[1]),
        }
    }

    pub fn is_fully_contained(&self) -> bool {
        Self::_is_a_fully_contained_in_b(&self.range_a, &self.range_b)
            || Self::_is_a_fully_contained_in_b(&self.range_b, &self.range_a)
    }

    pub fn is_overlapping(&self) -> bool {
        Self::_is_a_overlapping_b(&self.range_a, &self.range_b)
            || Self::_is_a_overlapping_b(&self.range_b, &self.range_a)
    }

    fn _is_a_fully_contained_in_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
        a.start() >= b.start() && a.end() <= b.end()
    }

    fn _is_a_overlapping_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
        a.start().max(b.start()) <= a.end().min(b.end())
    }
}

struct Assignments {
    pairs: Vec<AssignmentPair>,
}

impl Assignments {
    pub fn from_str(s: &str) -> Self {
        Self {
            pairs: s.lines().map(AssignmentPair::from_str).collect::<Vec<_>>(),
        }
    }

    pub fn fully_contained_count(&self) -> usize {
        self.pairs
            .iter()
            .filter(|assignment| assignment.is_fully_contained())
            .count()
    }

    pub fn overlapping_count(&self) -> usize {
        self.pairs
            .iter()
            .filter(|assignment| assignment.is_overlapping())
            .count()
    }
}

pub fn run_part1() -> String {
    Assignments::from_str(INPUT)
        .fully_contained_count()
        .to_string()
}

pub fn run_part2() -> String {
    Assignments::from_str(INPUT).overlapping_count().to_string()
}

#[cfg(test)]
mod test {
    use super::{run_part1, run_part2, Assignments};

    const SAMPLE: &str = include_str!("day04/sample.txt");

    #[test]
    fn test_sample() {
        let assignments = Assignments::from_str(SAMPLE);
        assert_eq!(assignments.fully_contained_count(), 2);
        assert_eq!(assignments.overlapping_count(), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(), "582");
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2(), "893");
    }
}
