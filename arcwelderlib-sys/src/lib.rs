use std::path::PathBuf;

/// Returns the path to the built ArcWelder executable
fn arcwelder_exe_path() -> PathBuf {
    PathBuf::from(env!("ARCWELDER_PATH"))
}

/// publicly callable command.
pub fn arcwelder(input_file: &str, output_file: &str) -> () {
    let arcwelder = arcwelder_exe_path();
    let status = std::process::Command::new(arcwelder)
        .args([input_file, output_file])
        .status()
        .expect("Failed to execute ArcWelder");
}
