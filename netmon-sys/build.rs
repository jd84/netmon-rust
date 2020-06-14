use cmake;

fn main() {
    let dst = cmake::build("netmon");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=netmon");
}
