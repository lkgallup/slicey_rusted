use crate::gcode::GcodeWriter;
use crate::geometry::{Edges, STLMesh};
use crate::settings::{FloatOrVecOfFloats, Settings};

pub struct Slicer {
    settings: Settings,
    stl_meshes: Vec<STLMesh>
}

impl Slicer {
    pub fn new(
        settings: Settings, 
        stl_meshes: Vec<STLMesh>
    ) -> Self {
        Self {
            settings: settings,
            stl_meshes: stl_meshes
        }
    }

    pub fn layer_heights(&self, stl: &STLMesh) -> Vec<f32> {
        let bb = stl.bounding_box();
        let mut heights = vec![self.settings.layer_height.layer_0_height];

        let total_height = bb.z_max - bb.z_min - heights[0];

        let layer_height = match &self.settings.layer_height.layer_n_height {
            FloatOrVecOfFloats::Float(x) => x,
            FloatOrVecOfFloats::VecOfFLoats(_x) => panic!("Got a list for layer n heights")
        };

        let n_layers = match &self.settings.layer_height.layer_n_height {
            FloatOrVecOfFloats::Float(x) => total_height / x,
            FloatOrVecOfFloats::VecOfFLoats(_x) => panic!("Got a list for layer n heights")
        };
        // round up to nearest layer
        let n_layers = n_layers.ceil() as u32;

        // below doesn't support variable layer height case
        for _layer in 0..n_layers {
            heights.push(*layer_height);
        }
        heights
    }

    pub fn perimeters(&self, stl: &STLMesh, z: f32) -> () {
        let tol = 1e-6 as f32;
        let tris = stl.triangles();
        let perimeter_edges = Edges::new();
        // let perimeter_edges = Vec
        // need to iterate over triangles
        // interpolate the z value in the triangle
        // see if it intersects
        // if it does add a line in the projected plane
    }

    pub fn slice(&self, gcode_file: &str) -> () {
        let mut gcode_writer = GcodeWriter::new(gcode_file);
        gcode_writer.write_header(&self.settings);

        for stl in &self.stl_meshes {
            println!("Slicing stl file {:?}", stl.file_name());
            println!("{}", self.settings);
            println!("Generating layer heights");
            let zs = self.layer_heights(&stl);
            
            for (n, z) in zs.iter().enumerate() {
                gcode_writer.write_layer_change(
                    n.try_into().unwrap(), *z, 'F', 1200. // TODO
                );
                // TODO skirt/brim
                println!("Generating perimeters for layer {}", n);
                let _ = self.perimeters(&stl, *z);
                // TODO infill
            }
        }
    }
}
