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
        let mut pixels = Vec::new();
        let cubes = Vec::new();

        for pixel in 0..(width * height) {
            pixels.push(background)
        }

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

    pub fn render(&mut self) -> *const Pixel {
        // TODO modify pixel according to cubes
        for cube in &self.cubes {
            let mut count = 0;
            for depth_pixel in cube.render(self.width, self.height) {
                self.pixels[count] = depth_pixel.pixel;
                count += 1;
            }
        }

        self.pixels.as_ptr()
    }
}
