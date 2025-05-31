use serde::Deserialize;
// use std::fmt::{Debug, Display};
use std::fmt;
use std::fs::File;
use std::io::BufReader;

// helpers
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum IntOrVecOfInts {
    Int(i32),
    VecOfInts(Vec<i32>)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum FloatOrVecOfFloats {
    Float(f32),
    VecOfFLoats(Vec<f32>)
}

#[derive(Clone, Debug, Deserialize)]
pub struct BrimSettings {

}

#[derive(Clone, Debug, Deserialize)]
pub struct InfillSettings {

}

// need to check that at least one of 
// layer_n_height or layer_n_heights is Some
#[derive(Clone, Debug, Deserialize)]
pub struct LayerHeightSettings {
    pub layer_0_height: f32,
    pub layer_n_height: FloatOrVecOfFloats
}

#[derive(Clone, Debug, Deserialize)]
pub struct PerimeterSettings {
    pub layer_0_feed_rate: f32,
    pub layer_n_feed_rate: FloatOrVecOfFloats,
    pub layer_0_wall_line_count: u32,
    pub layer_n_wall_line_count: IntOrVecOfInts
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkirtSettings {

}

#[derive(Clone, Debug, Deserialize)]
pub struct XYResolution {
    pub x_pixels: i32,
    pub y_pixels: i32,
    pub x_pixel_resolution: f32,
    pub y_pixel_resolution: f32
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub brim: Option<BrimSettings>,
    pub infill: Option<InfillSettings>,
    pub layer_height: LayerHeightSettings,
    pub material: String,
    pub name: String,
    pub perimeter: Option<PerimeterSettings>,
    pub skirt: Option<SkirtSettings>,
    pub xy_resolution: Option<XYResolution>
}

impl Settings {
    pub fn new(file_name: &str) -> Self {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let json_settings: Settings = 
            serde_json::from_reader(reader).unwrap();
        json_settings
    }

    pub fn to_gcode_comment(&self) -> String {
        let s = format!("{}", self);
        s.lines()
            .map(|line| format!("; {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "{:?}\n", self.name);
        let _ = write!(f, "Material = {:?}\n", self.material);
        let _ = write!(f, "{:#?}\n", self.brim);
        let _ = write!(f, "{:#?}\n", self.infill);
        let _ = write!(f, "{:#?}\n", self.layer_height);
        let _ = write!(f, "{:#?}\n", self.perimeter);
        let _ = write!(f, "{:#?}\n", self.skirt);
        write!(f, "{:#?}\n", self.xy_resolution)
    }
}