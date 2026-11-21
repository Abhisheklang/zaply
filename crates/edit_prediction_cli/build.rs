fn main() {
    println!("cargo:rerun-if-changed=../zaply/Cargo.toml");
    let cargo_toml =
        std::fs::read_to_string("../zaply/Cargo.toml").expect("Failed to read crates/zaply/Cargo.toml");
    let version = cargo_toml
        .lines()
        .find(|line| line.starts_with("version = "))
        .expect("Version not found in crates/zaply/Cargo.toml")
        .split('=')
        .nth(1)
        .expect("Invalid version format")
        .trim()
        .trim_matches('"');
    println!("cargo:rustc-env=ZAPLY_PKG_VERSION={}", version);
}
