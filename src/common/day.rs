use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Day(u32);

impl TryFrom<u32> for Day {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !(1..=25).contains(&value) {
            Err(format!("Invalid Advent of Code day {}", value))
        } else {
            Ok(Day(value))
        }
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>()
            .unwrap_or_else(|_| panic!("Invalid Advent of Code day: {}", s))
            .try_into()
    }
}
