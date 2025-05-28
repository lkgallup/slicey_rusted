#[cfg(feature = "arcwelder")]
use arcwelderlib_sys::arcwelder;

use clap::Parser;
use slicey::{
    geometry::STLMesh,
    settings::Settings,
    slicer::Slicer
};

// CLIP parser
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
// #[command(name = "slicey", about = "A slicey CLI example", version)]
struct CLIArgs {
    /// Path to gcode to write file to
    #[arg(short, long)]
    gcode_file: String,
    /// Paths to stl file/files
    #[arg(short, long, value_delimiter = ',')]
    stl_files: Vec<String>,
    /// Path to settings file
    #[arg(long)]
    settings_file: String,
    /// Whether or not to use arcwelder. Requires cargo build --features arcwelder
    #[arg(long)]
    arcwelder: bool
}

fn main() {
    let args = CLIArgs::parse();
    println!("STL file      = {:?}", args.stl_files);
    let settings = Settings::new(&args.settings_file);
    let mut stl_meshes: Vec<STLMesh> = args.stl_files
        .iter()
        .map(|x| STLMesh::new(x.clone()))
        .collect();
    let slicer = Slicer::new(settings, stl_meshes);
    let _ = slicer.slice(&args.gcode_file);

    // arcwelder stuff
    let arcwelder_enabled = cfg!(feature = "arcwelder");
    if arcwelder_enabled {
        println!("ArcWelder feature is enabled.");
    } else {
        println!("ArcWelder feature is NOT enabled.");
    }

    #[cfg(feature = "arcwelder")]
    if args.arcwelder {
        arcwelder(&args.gcode_file, &args.gcode_file)
    }
}
