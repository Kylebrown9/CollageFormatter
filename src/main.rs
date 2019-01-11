extern crate collage_formatter_lib;

use std::path::Path;
use std::fs;

use image::{ DynamicImage, RgbaImage };

use collage_formatter_lib::collage_builder::*;

fn main() {
    let mut tile_images: Vec<DynamicImage> = Vec::new();

    for entry in fs::read_dir(".").expect("Could not read current directory") {
        let dir = entry.expect("Issue reading dir entry");

        if dir.file_name().into_string().unwrap().starts_with("scope") {
            tile_images.push(image::open(dir.path().as_path()).unwrap());
        }
    }

    let collages: Vec<RgbaImage> = create_collages(tile_images);

    for (index, collage) in collages.iter().enumerate() {
        let path_name = format!("collage_{}.png", index);
        let _ = collage.save(&Path::new(&path_name)).expect("Saving image failed");
    }
}