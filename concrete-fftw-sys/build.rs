use std::env::var;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::process::Command;

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
    // ========================================================================= Check configuration
    if cfg!(windows) {
        panic!("Windows platform is not supported.")
    }
    if cfg!(macos) && cfg!(feature = "mkl") {
        panic!("Mkl is not supported in the macos platform.")
    }
    // ================================================================================ Copy sources
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
    )
    .expect("Failed to copy sources.");

    // ===================================================================================== Compile
    let mut configure = Command::new(canonicalize(out_src_dir.join("configure")).unwrap());
    configure
        .arg("--with-pic")
        .arg("--enable-static")
        .arg("--enable-avx")
        .arg("--enable-avx2")
        .arg("--enable-sse2")
        .arg("--enable-generic-simd128")
        .arg("--enable-generic-simd256")
        .arg("--disable-doc")
        .arg(format!("--prefix={}", out_dir.display()))
        .current_dir(&out_src_dir);
    run(&mut configure);
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&out_src_dir));
    run(Command::new("make")
        .arg("install")
        .current_dir(&out_src_dir));

    // run(Command::new("make distclean").current_dir(&out_src_dir));
    let mut configure = Command::new(canonicalize(out_src_dir.join("configure")).unwrap());
    configure
        .arg("--with-pic")
        .arg("--enable-static")
        .arg("--enable-single")
        .arg("--enable-avx")
        .arg("--enable-avx2")
        .arg("--enable-sse")
        .arg("--enable-sse2")
        .arg("--enable-generic-simd128")
        .arg("--enable-generic-simd256")
        .arg("--disable-doc")
        .arg(format!("--prefix={}", out_dir.display()))
        .current_dir(&out_src_dir);
    run(&mut configure);
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&out_src_dir));
    run(Command::new("make")
        .arg("install")
        .current_dir(&out_src_dir));

    // ================================================================================== Emit flags
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=fftw3");
    println!("cargo:rustc-link-lib=static=fftw3f");
}
