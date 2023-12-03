#![feature(
    iter_array_chunks,
    iter_next_chunk,
    extract_if,
    let_chains,
    iterator_try_collect,
    hash_set_entry
)]
use std::io::Write;
use std::process::{Command, Stdio};
use std::{collections::HashMap, time::Instant};

mod days;
use days::*;

include!(concat!(env!("OUT_DIR"), "/loc.rs"));

fn main() -> anyhow::Result<()> {
    let loc: HashMap<&str, u32> = HashMap::from(PAIRS);

    let name = env!("CARGO_PKG_NAME");
    let rep = "#".repeat(name.len() + 6);
    println!("{rep}");
    println!("## {name} ##");
    println!("{rep}");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let func = include!("_match_days.rs");

        println!("\n=== Day {arg:0>2} ===");
        println!("From {}/day/{arg}", env!("CARGO_PKG_HOMEPAGE"));

        let instant = Instant::now();
        let result = func();
        let elapsed = instant.elapsed().as_micros();

        if let Ok((part_1, part_2)) = result {
            println!("Part 1: {part_1}\nPart 2: {part_2}\nTook {elapsed} μs");
            if let (_, Some(remaining)) = args.size_hint()
                && remaining == 0
            {
                if part_2.has_value() {
                    if let Ok(_) = set_clip(format!("{part_2}")) {
                        println!(
                        "The solution to part 2, `{part_2}`, has been copied to your clipboard."
                    );
                    }
                } else if part_1.has_value() {
                    if let Ok(_) = set_clip(format!("{part_1}")) {
                        println!(
                        "The solution to part 1, `{part_1}`, has been copied to your clipboard."
                    );
                    }
                }
            }

            let lines = loc.get(format!("day{arg}.rs").as_str()).unwrap();
            println!("Lines of code: {lines}");
        } else {
            eprintln!("Error: {}", result.unwrap_err());
        }
    }
    Ok(())
}

fn set_clip(s: String) -> anyhow::Result<()> {
    let mut command = Command::new("clipman")
        .arg("store")
        .stdin(Stdio::piped())
        .spawn()?;
    write!(command.stdin.take().unwrap(), "{s}")?;
    Ok(())
}
