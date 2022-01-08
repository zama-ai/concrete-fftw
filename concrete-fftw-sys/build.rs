use path_absolutize::Absolutize;
use std::env::var;
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

    // build standard precision
    let build_dir = out_src_dir.absolutize().unwrap().join("build");
    let _ = std::fs::remove_dir_all(&build_dir);
    std::fs::create_dir(&build_dir).unwrap();
    let mut cmake = Command::new("cmake");
    cmake
        .arg("-DCMAKE_POSITION_INDEPENDENT_CODE=ON")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DBUILD_TESTS=OFF")
        .arg("-DENABLE_SSE=ON")
        .arg("-DENABLE_SSE2=ON")
        .arg("-DENABLE_AVX=ON")
        .arg("-DENABLE_AVX2=ON")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()))
        .arg("-G")
        .arg("Ninja")
        .arg("..")
        .current_dir(&build_dir);
    run(&mut cmake);
    run(Command::new("cmake")
        .arg("--build")
        .arg(".")
        .arg("--parallel")
        .arg(var("NUM_JOBS").unwrap())
        .current_dir(&build_dir));
    run(Command::new("ninja")
        .arg("install")
        .current_dir(&build_dir));

    // build single precision
    let build_dir = out_src_dir.absolutize().unwrap().join("buildf");
    let _ = std::fs::remove_dir_all(&build_dir);
    std::fs::create_dir(&build_dir).unwrap();
    let mut cmake = Command::new("cmake");
    cmake
        .arg("-DCMAKE_POSITION_INDEPENDENT_CODE=ON")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DBUILD_TESTS=OFF")
        .arg("-DENABLE_FLOAT=ON")
        .arg("-DENABLE_SSE=ON")
        .arg("-DENABLE_SSE2=ON")
        .arg("-DENABLE_AVX=ON")
        .arg("-DENABLE_AVX2=ON")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()))
        .arg("-G")
        .arg("Ninja")
        .arg("..")
        .current_dir(&build_dir);
    run(&mut cmake);
    run(Command::new("cmake")
        .arg("--build")
        .arg(".")
        .arg("--parallel")
        .arg(var("NUM_JOBS").unwrap())
        .current_dir(&build_dir));
    run(Command::new("ninja")
        .arg("install")
        .current_dir(&build_dir));

    // ================================================================================== Emit flags
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=fftw3");
    println!("cargo:rustc-link-lib=static=fftw3f");
}
