use std::ffi::OsStr;

fn main() {
    #[cfg(feature = "asan")]
    let flags: &[&OsStr] = &[OsStr::new("-fsanitize=address")];

    #[cfg(not(feature = "asan"))]
    let flags: &[&OsStr] = &[];

    cc::Build::new().file("src/c_lib.c").flags(flags).compile("c_lib");
}
