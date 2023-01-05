/// Run day.
#[macro_export]
macro_rules! run_day_macro {
    ($day:literal) => {{
        paste::paste! {
            use $crate::days::[<day $day>];
            use owo_colors::OwoColorize;

            let (d1_output, d1_elapsed) = {
                let now = std::time::Instant::now();
                let output = [<day $day>]::run_part1();
                (output, now.elapsed().as_millis().to_string())
            };
            let (d2_output, d2_elapsed) = {
                let now = std::time::Instant::now();
                let output = [<day $day>]::run_part2();
                (output, now.elapsed().as_millis().to_string())
            };

            println!(
                "📆 {} : 🌓 {} ({}ms) — 🌕 {} ({}ms)",
                $day.bold().blue(), d1_output.green(), d1_elapsed.yellow(), d2_output.green(), d2_elapsed.yellow()
            );
        }
    }};
}
