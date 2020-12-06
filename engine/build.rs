// build.rs
// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-search=assets\\lib", );
    }
}