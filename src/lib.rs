use std::error;
use std::fs;
use crate::rendering::{Scene, Camera, Renderable, Material, Color, Light};
use crate::shape::{Shape, Sphere, Point};

mod shape;
mod vertors;
mod rendering;
mod traits;

pub struct Config {
    pub scene_path: String,
    pub output_path: String,
    pub nb_pass: u8
}

impl Config {
    pub fn new(scene_path: String, output_path: String, nb_pass: u8) -> Config {
        Config {
            scene_path,
            output_path,
            nb_pass
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    println!("Using scene: {}", config.scene_path);
    println!("Writing to {}", config.output_path);
    println!("Number of passes: {}", config.nb_pass);

    let file_content = fs::read_to_string(config.scene_path)?;

    let scene: Scene = match serde_json::from_str(&file_content) {
        Ok(parsed) => parsed,
        Err(e) => return Err(Box::new(e))
    };
    if let Err(e) = rendering::render(config.nb_pass, scene, config.output_path) {
        return Err(Box::new(e));
    }
    Ok(())
}
