fn main() {
    // TODO: separate buils?
    cc::Build::new()
        .file("upng/upng.c")
        .file("miniaudio/audio.c")
        .compile("hrust_libs");
}
