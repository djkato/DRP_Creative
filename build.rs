#[cfg(windows)]
extern crate windres;

#[cfg(windows)]
use windres::Build;

#[cfg(windows)]
fn main() {
    Build::new().compile("./icon/tray-icon.rc").unwrap();
}
