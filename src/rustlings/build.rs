use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let exercises_path = Path::new(&manifest_dir).join("exercises");
    let canonical_path = fs::canonicalize(&exercises_path).unwrap();
    println!("cargo:rustc-env=EXERCISES_PATH={}", canonical_path.display());
}
