use std::collections::HashMap;

use clap::Parser;

use super::day::Day;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(Default)]
pub struct DayMap<'a> {
    map: HashMap<Day, &'a dyn Fn()>,
}

impl<'a> DayMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(mut self, day: u32, f: &'a dyn Fn()) -> Self {
        self.map.insert(Day::try_from(day).unwrap(), f);
        self
    }

    pub fn run_day(&self, day: Day) {
        self.map[&day]();
    }

    pub fn run_all(&self) {
        let count = self.map.len();

        for day_nb in 1..=count {
            let day = Day::try_from(day_nb as u32).unwrap();
            self.run_day(day);
        }
    }
}

impl Args {
    pub fn execute(self, day_map: DayMap) {
        match self.cmd {
            Subcommand::RunDay(cmd) => day_map.run_day(cmd.day),
            Subcommand::RunAll(_cmd) => day_map.run_all(),
        }
    }
}

#[derive(Parser)]
enum Subcommand {
    RunDay(RunDayCommand),
    RunAll(RunAllCommand),
}

#[derive(Parser)]
pub struct RunDayCommand {
    day: Day,
}

#[derive(Parser)]
pub struct RunAllCommand;
