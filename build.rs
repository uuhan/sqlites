fn main() {
    println!("cargo:rerun-if-changed=c/vfs.c");
    println!("cargo:rerun-if-changed=c/vfs.h");

    let mut build = cc::Build::new();
    build
        .flag_if_supported("-Wmissing-declarations")
        .flag_if_supported("-Wmissing-prototypes")
        .flag_if_supported("-Wstrict-prototypes")
        .flag_if_supported("-Wundef");

    // from libsqlite3-sys
    let dep_includes = std::env::var("DEP_SQLITE3_INCLUDE").expect("sqlite3 include");

    #[cfg(target_os = "linux")]
    build.define("_GNU_SOURCE", None);
    build
        .define("SQLITE_DISABLE_DIRSYNC", None)
        .include(&dep_includes)
        .file("c/vfs.c")
        .opt_level(2)
        .compile("boot");
}
