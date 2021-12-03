mod days;
use std::time::Instant;

use days::*;

fn main() -> anyhow::Result<()> {
    for day in std::env::args().skip(1) {
        let func = match day.as_str() {
            "1" => day01::main,
            "2" => day02::main,
            "3" => day03::main,
            "4" => day04::main,
            "5" => day05::main,
            "6" => day06::main,
            "7" => day07::main,
            "8" => day08::main,
            "9" => day09::main,
            "10" => day10::main,
            "11" => day11::main,
            "12" => day12::main,
            "13" => day13::main,
            "14" => day14::main,
            "15" => day15::main,
            "16" => day16::main,
            "17" => day17::main,
            "18" => day18::main,
            "19" => day19::main,
            "20" => day20::main,
            "21" => day21::main,
            "22" => day22::main,
            "23" => day23::main,
            "24" => day24::main,
            "25" => day25::main,
            _ => panic!("Day not implemented."),
        };

        println!("\n=== Day {:02} ===", day);

        let instant = Instant::now();
        let result = func();
        let elapsed = instant.elapsed().as_micros();

        if let Ok((part_1, part_2)) = result {
            println!("Part 1: {}\nPart 2: {}", part_1, part_2);
            println!("Took {} μs", elapsed);
        } else {
            eprintln!("Error: {}", result.unwrap_err());
        }
    }
    Ok(())
}
