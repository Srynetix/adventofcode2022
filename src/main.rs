use adventofcode2022::{run_day_macro, Args, DayMap};
use clap::Parser;

fn main() {
    color_eyre::install().expect("Could not install color-eyre.");

    let args = Args::parse();
    args.execute(
        DayMap::new()
            .add(1, &|| run_day_macro!("01"))
            .add(2, &|| run_day_macro!("02"))
            .add(3, &|| run_day_macro!("03")),
    )
}
