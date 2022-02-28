use wasm_bindgen::prelude::*;
use crate::render::*;
use crate::pixel::*;
use crate::cube::*;

extern crate web_sys;

mod cube;
mod pixel;
mod render;
mod utils;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }


#[wasm_bindgen]
pub struct Prolocube {
    canvas: Canvas,
}

#[wasm_bindgen]
impl Prolocube {
    pub fn new(width: i32,  bg_r: u8, bg_g: u8, bg_b: u8, bg_a: u8) -> Prolocube {
        let background = Pixel {
            r : bg_r,
            g : bg_g,
            b : bg_b,
            a : bg_a,
        };
        let mut canvas = Canvas::new(width, width, background);
        let cube = Cube::new((width / 2) as f32, (width / 2) as f32, (width / 2) as f32, (width / 2) as f32);

        canvas.add_cube(cube);

        Prolocube {
            canvas,
        }
    }

    pub fn rotate(&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.canvas.rotate(roll, pitch, yaw);
    }

    pub fn render(&mut self) -> *const Pixel {
        self.canvas.render()
    }
}
