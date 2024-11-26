#[cfg(windows)]
extern crate windres;
use windres::Build;
fn main() {
    Build::new().compile("assets/icon-256.rc").unwrap();
}
