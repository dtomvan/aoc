use loc::count;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // TODO: get all module names and insert "mod dayn;\nusedayn::main as dayn;" for every day.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src/days");
    let mut map = HashMap::new();

    for file in std::fs::read_dir(path).unwrap() {
        let file = file.unwrap();

        if file.metadata().unwrap().is_file() {
            map.insert(
                file.file_name(),
                count(file.path().as_os_str().to_str().unwrap()).code,
            );
        }
    }

    let dest_path = Path::new(&out_dir).join("loc.rs");
    fs::write(
        &dest_path,
        format!(
            "const PAIRS: [(&str, u32); {}] = {:?};",
            map.len(),
            map.iter().collect::<Vec<_>>()
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=src");
}
