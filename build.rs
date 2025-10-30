fn main() {
    cc::Build::new()
        .file("upng/upng.c")
        .compile("upng");
}
