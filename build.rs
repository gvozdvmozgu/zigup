fn main() {
    let target = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);

    println!("cargo:rustc-env=HOST_TRIPLE={}", target);
    println!("cargo:rerun-if-changed=build.rs");
}
