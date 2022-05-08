
fn main() {
    cc::Build::new()
        .file("src/opt_sin.c")
        .flag_if_supported("-ffast-math")
        .compile("opt_sin");
}