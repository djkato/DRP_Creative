use windres::Build;

fn main() {
    Build::new().compile("./icon/tray-icon.rc").unwrap();
}
