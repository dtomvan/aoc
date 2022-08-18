// TODO: DRY I guess
use std::{collections::HashMap, time::Instant};

mod days;
use days::*;

include!(concat!(env!("OUT_DIR"), "/loc.rs"));

fn main() -> anyhow::Result<()> {
    // TODO: Uncomment in December
    // let loc: HashMap<&str, u32> = HashMap::from(PAIRS);

    // for arg in std::env::args().skip(1) {
    //     let func = include!("_match_days.rs");
    //
    //     println!("\n=== Day {:0>2} ===", arg);
    //
    //     let instant = Instant::now();
    //     let result = func();
    //     let elapsed = instant.elapsed().as_micros();
    //
    //     if let Ok((part_1, part_2)) = result {
    //         println!("Part 1: {}\nPart 2: {}", part_1, part_2);
    //         println!("Took {} μs", elapsed);
    //
    //         let lines = loc.get(format!("day{:0>2}.rs", arg).as_str()).unwrap();
    //         println!("Lines of code: {}", lines);
    //     } else {
    //         eprintln!("Error: {}", result.unwrap_err());
    //     }
    // }
    Ok(())
}
