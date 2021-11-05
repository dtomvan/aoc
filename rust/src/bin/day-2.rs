use std::io::{self, BufRead};
fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        dbg!(line?);
    }
    Ok(())
}
