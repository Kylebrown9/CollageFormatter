use image::imageops::FilterType::Lanczos3;
use image::imageops::overlay;

use image::{ DynamicImage, ImageBuffer, RgbaImage, Rgba };

#[derive(Clone)]
struct Point {
    x: u32,
    y: u32
}

const COLLAGE_W: u32 = 2550;
const COLLAGE_H: u32 = 3300;

const FOOTER_H: u32 = 320;

const TILE_W: u32 = 1050;
const TILE_H: u32 = 654;

const GAP_W: u32 = (COLLAGE_W - (2 * TILE_W)) / 3;
const GAP_H: u32 = (COLLAGE_H - (4 * TILE_H) - FOOTER_H) / 5;

const LEFT_X: u32 = GAP_W;
const RIGHT_X: u32 = GAP_W * 2 + TILE_W;

const HEIGHT_INC: u32 = GAP_H + TILE_H;

const LOCATIONS: [Point; 8] = [
    Point { x: LEFT_X,  y: GAP_H },
    Point { x: RIGHT_X, y: GAP_H },
    Point { x: LEFT_X,  y: GAP_H + HEIGHT_INC },
    Point { x: RIGHT_X, y: GAP_H + HEIGHT_INC },
    Point { x: LEFT_X,  y: GAP_H + HEIGHT_INC * 2 },
    Point { x: RIGHT_X, y: GAP_H + HEIGHT_INC * 2 },
    Point { x: LEFT_X,  y: GAP_H + HEIGHT_INC * 3 },
    Point { x: RIGHT_X, y: GAP_H + HEIGHT_INC * 3 }
];

pub fn create_collages(tiles: Vec<DynamicImage>) -> Vec<RgbaImage> {
    let mut collages: Vec<RgbaImage> = Vec::new();   

    for chunk in tiles.chunks(8) {
        let mut collage: RgbaImage = ImageBuffer::from_pixel(COLLAGE_W, COLLAGE_H, Rgba([255,255,255,255]));

        for (index, tile) in chunk.iter().enumerate() {
            let mut tile_clone = tile.clone();

            tile_clone.invert();
            let resized = tile_clone.resize(TILE_W, TILE_H, Lanczos3);

            overlay(&mut collage, &resized.to_rgba(), LOCATIONS[index].x, LOCATIONS[index].y);
        }

        collages.push(collage);
    }

    collages
}