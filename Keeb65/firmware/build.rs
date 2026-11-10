fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("Cargo project directory");
    println!("cargo:rerun-if-changed=keyboard.toml");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-search={root}");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}
