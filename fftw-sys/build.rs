use std::env::var;
use std::fs::canonicalize;
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use bindgen::EnumVariation;

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn main() {
    // Copy fftw sources to the compilation folder:
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let src_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("fftw-3.3.8");
    let out_src_dir = out_dir.join("src");
    fs_extra::dir::copy(
        src_dir.as_path(),
        &out_src_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
            content_only: false,
        },
    );
    run(
        Command::new(canonicalize(src_dir.join("configure")).unwrap())
            .arg("--with-pic")
            .arg("--enable-static")
            .arg("--disable-doc")
            .arg("--enable-quad-precision")
            .arg(format!("--prefix={}", out_dir.display()))
            .current_dir(&src_dir),
    );
    run(
        Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_dir)
    );
    run(
        Command::new("make").arg("install").current_dir(&src_dir)
    );
    let bindings = bindgen::Builder::default()
        .header(out_dir.join("include/fftw3.h").into_os_string().into_string().unwrap())
        .use_core()
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .whitelist_type("^fftw.*")
        .whitelist_var("^FFTW.*")
        .whitelist_function("^fftw.*")
        .blacklist_type("fftwl_complex")
        //.blacklist_type("FILE")
        //.blacklist_function("fftwl_.*")
        //.blacklist_type("_.*")
        //.blacklist_type("fftw_.*_complex")
        .default_enum_style(EnumVariation::Rust{non_exhaustive:false})
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=fftw3q");
}
