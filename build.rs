use windres::Build;

fn main() {
    Build::new()
        .compile("app.rc")
        .unwrap();
}
