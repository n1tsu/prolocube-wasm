use crate::pixel::*;
use crate::cube::*;

pub struct Canvas {
    width: i32,
    height: i32,
    background: Pixel,
    pixels: Vec<Pixel>,
    cubes: Vec<Cube>,
}

impl Canvas {
    pub fn new(width: i32, height: i32, background: Pixel) -> Canvas {
        let pixels = Vec::new();
        let cubes = Vec::new();
        Canvas {
            width,
            height,
            background,
            pixels,
            cubes,
        }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }

    pub fn rotate(&mut self, roll: f32, pitch: f32, yaw: f32) {
        for cube in &mut self.cubes {
            cube.rotate(roll, pitch, yaw);
        }
    }

    pub fn render(&self) -> *const Pixel {
        // TODO modify pixel according to cubes
        self.pixels.as_ptr()
    }
}
