use std::fs;
use std::path::Path;

#[cfg(not(target_os = "windows"))]
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let db_path = Path::new(&out_dir).join("tasks.db");
    fs::copy("var/db/tasks.db", db_path).expect("Failed to copy DB file");
}