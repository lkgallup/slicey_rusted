pub mod dlp_slicer;
pub mod fff_slicer;

pub use dlp_slicer::DLPSlicer;
pub use fff_slicer::FFFSlicer;

use crate::geometry::STLMesh;
use crate::settings::{FloatOrVecOfFloats, Settings};

/// trait for shared behavior between slicers
pub trait Slicer {
    fn layer_heights(&self, settings: &Settings, stl: &STLMesh) -> Vec<f32> {
        let bb = stl.bounding_box();
        let mut heights = vec![settings.layer_height.layer_0_height];

        let total_height = bb.z_max - bb.z_min - heights[0];

        let layer_height = match &settings.layer_height.layer_n_height {
            FloatOrVecOfFloats::Float(x) => x,
            FloatOrVecOfFloats::VecOfFLoats(_x) => panic!("Got a list for layer n heights")
        };

        let n_layers = match &settings.layer_height.layer_n_height {
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

    fn slice(&self, some_file: &str);
}
