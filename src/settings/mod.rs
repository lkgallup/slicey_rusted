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
pub struct Settings {
    pub brim: BrimSettings,
    pub infill: InfillSettings,
    pub layer_height: LayerHeightSettings,
    pub material: String,
    pub name: String,
    pub perimeter: PerimeterSettings,
    pub skirt: SkirtSettings
}

impl Settings {
    pub fn new(file_name: &str) -> Self {
        let file = File::open(file_name).unwrap();
        // let mut data = String::new();
        // file.read_to_string(&mut data).unwrap();
        let reader = BufReader::new(file);
        let json_settings: Settings = 
            serde_json::from_reader(reader).unwrap();
        json_settings
    }

    // pub fn layer_heights(&self) -> Vec<f32> {
    //     let heights = vec![self.layer_height.layer_0_height];

    //     heights
    // }

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
        let _ = write!(f, "{:#?}\n", self.infill);
        let _ = write!(f, "{:#?}\n", self.layer_height);
        write!(f, "{:#?}\n", self.perimeter)
    }
}