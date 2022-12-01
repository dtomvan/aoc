#![feature(iter_next_chunk, drain_filter, let_chains, iterator_try_collect, hash_set_entry)]
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
    for arg in std::env::args().skip(1) {
        let func = include!("_match_days.rs");

        println!("\n=== Day {arg:0>2} ===");
        println!("From {}/day/{arg}", env!("CARGO_PKG_HOMEPAGE"));

        let instant = Instant::now();
        let result = func();
        let elapsed = instant.elapsed().as_micros();

        if let Ok((part_1, part_2)) = result {
            println!("Part 1: {}\nPart 2: {}", part_1, part_2);
            println!("Took {} μs", elapsed);

            let lines = loc.get(format!("day{}.rs", arg).as_str()).unwrap();
            println!("Lines of code: {}", lines);
        } else {
            eprintln!("Error: {}", result.unwrap_err());
        }
    }
    Ok(())
}
