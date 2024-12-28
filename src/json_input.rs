use serde::{Deserialize, Serialize};

// ATTENTION - some fields here are missing as those are not revelant for the planetgen, for example, atmo

#[derive(Debug, Serialize, Deserialize)]
pub struct InputVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputHeightModifier {
    pub image_path: String,
    pub direction: InputVector3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputColorModifier {
    pub image_path: String,
    pub direction: InputVector3,
    pub size: f64,
    pub rotation: f64,
    pub influence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputTerrainGeneration {
    pub fbm_scale: f64,
    pub fbm_iterations: u8,
    pub fbm_iteration_scale_coefficient: f64,
    pub fbm_iteration_weight_coefficient: f64,
    pub fbm_final_power: f64,
    pub height_modifiers: Vec<InputHeightModifier>,
    pub color_modifiers: Vec<InputColorModifier>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InputBiomeModifier {
    Latitude,
    Tidal,
    Random,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputBiome {
    pub id: u32,
    pub min_altitude: f64,
    pub max_altitude: f64,
    pub min_modifier: f64,
    pub max_modifier: f64,
    pub color: InputVector3,
    pub roughness: f64,
    pub erosion_strength: f64,
    pub deposition_strength: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputTerrain {
    pub radius: f64,
    pub min_height: f64,
    pub max_height: f64,
    pub biome_modifier: InputBiomeModifier,
    pub biomes: Vec<InputBiome>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputWater {
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputPlanetGenConfig {
    pub out_dir: String,

    pub subdivide_initial: u8,
    pub subdivide_level1: u8,
    pub subdivide_level2: u8,
    pub subdivide_level3: u8,

    pub erosion_iterations: u16,
    pub erosion_droplets_count: u16,

    pub cube_map_resolution: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputCelestialBodyDefinition {
    pub id: String,
    pub terrain: Option<InputTerrain>,
    pub water: Option<InputWater>,
    pub generator_config: InputPlanetGenConfig,
}

pub fn parse_input_data(str: &str) -> InputCelestialBodyDefinition {
    let data: InputCelestialBodyDefinition = serde_json::from_str(str).unwrap();
    data
}
