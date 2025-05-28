use git2::Repository;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let arcwelder_src = PathBuf::from("vendor/ArcWelderLib");

    if !arcwelder_src.exists() {
        println!("Cloning ArcWelderLib...");
        Repository::clone("https://github.com/fieldOfView/ArcWelderLib.git", &arcwelder_src)
            .expect("Failed to clone ArcWelderLib");
    }

    // This assumes ArcWelderConsole is the name of the CMake target
    let dst = cmake::Config::new(&arcwelder_src)
        .build_target("ArcWelderConsole")
        .build();

    // Typical output location is build/ArcWelderConsole
    let exe_path = dst.join("build")
        .join("ArcWelderConsole")
        .join("ArcWelder");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_path = out_dir.join("ArcWelderConsole");

    fs::copy(&exe_path, &target_path).expect("Failed to copy ArcWelder executable");

    println!("cargo:rerun-if-changed=vendor/ArcWelderLib");
    println!("cargo:rustc-env=ARCWELDER_PATH={}", target_path.display());
}
