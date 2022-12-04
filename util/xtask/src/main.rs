use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use chrono::{Datelike, Utc};
use reqwest::blocking::Client;

macro_rules! bail {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
        std::process::exit(-1);
    };
}

const SESSION_ID: &str = include_str!("TOKEN");

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let task = args.next();
    if task.is_none() {
        bail!("Specify an xtask");
    }
    match task.unwrap().as_str() {
        "update" => {
            let year = args
                .next()
                .ok_or_else(|| anyhow!("Specify the year to update."))?
                .parse()?;
            let project_root = project_root(year)?;
            generate_match_days(
                &project_root.join("src/days"),
                &project_root,
                &project_root.join("inputs"),
            )?;
        }
        "year_dir" => {
            let (_, year) = date(args);
            println!("Root of {year}: {}", project_root(year)?.display());
        }
        "day" => {
            let (day, year) = date(args);
            let project_root = project_root(year)?;
            let days_dir = project_root.join("src/days");
            std::fs::create_dir_all(&days_dir).context("Couldn't create days dir")?;
            let mut day_file = File::create(days_dir.join(format!("day{day}.rs")))
                .context("Could not create today's file")?;
            writeln!(
                day_file,
                r#"use aoc_common::result::{{AocResult, done}};
pub fn main() -> AocResult {{
    // Part 1
    let input = include_str!("../../inputs/day-{day}.txt");
    // Part 2
    // TODO
    done((), ())
}}
"#
            )
            .context("Could not write to days file.")?;

            let inputs_dir = project_root.join("inputs");
            generate_match_days(&days_dir, &project_root, &inputs_dir)?;

            let client = Client::new();
            let res = client
                .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .header(
                    reqwest::header::COOKIE,
                    format!("session={SESSION_ID}").trim(),
                )
                .header(
                    reqwest::header::USER_AGENT,
                    "github.com/dtomvan/aoc by 18gatenmaker6@gmail.com",
                )
                .send()?
                .text()?;
            let mut day_input = File::create(inputs_dir.join(format!("day-{day}.txt")))?;
            writeln!(day_input, "{res}")?;
        }
        x => {
            bail!("No rules specified for target {x}.");
        }
    }
    Ok(())
}

fn date(mut args: impl Iterator<Item = String>) -> (u32, i32) {
    let day = args.next().and_then(|x| x.parse().ok());
    let year = args.next().and_then(|x| x.parse().ok());

    let date = Utc::now().with_timezone(&chrono_tz::EST);
    if date.month() != 12 && day.is_none() {
        bail!("Error: it is not currently December and no days have been given.");
    }
    let day = day.unwrap_or_else(|| date.day());
    let year = year.unwrap_or_else(|| date.year());
    (day, year)
}

fn project_root(year: i32) -> Result<PathBuf> {
    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest_root
        .parent()
        .and_then(Path::parent)
        .context("xtask dir should have a parent.")?
        .join("year")
        .join(year.to_string()))
}

fn generate_match_days(days_dir: &Path, project_root: &Path, inputs_dir: &Path) -> Result<()> {
    let mut day_module_names: Vec<_> = std::fs::read_dir(days_dir)?
        .filter_map(|x| x.ok())
        .filter_map(|x| {
            x.file_name()
                .to_string_lossy()
                .strip_prefix("day")
                .and_then(|x| x.strip_suffix(".rs").map(|x| x.to_string()))
        })
        .collect();
    day_module_names.sort_unstable();
    let day_matchers: Vec<_> = day_module_names
        .iter()
        .map(|x| format!(r#"	"{x}" => day{x}::main,"#))
        .collect();

    let mut match_days = File::create(project_root.join("src/_match_days.rs"))?;
    writeln!(
        match_days,
        r#"match arg.as_str() {{
{}
    _ => unimplemented!(),
}}"#,
        day_matchers.join("\n")
    )?;

    std::fs::create_dir_all(inputs_dir).context("Couldn't create inputs dir")?;

    let mut module_file = File::create(project_root.join("src/days/mod.rs"))?;
    let day_modules: Vec<_> = day_module_names
        .iter()
        .map(|x| format!("pub mod day{x};"))
        .collect();
    writeln!(module_file, "{}", day_modules.join("\n"))?;

    Ok(())
}
