use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub struct Cube {
    width: u32,
    anglex: u16,
    angley: u16,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Cube {
    pub fn new(width: u32) -> Cube {
        let mut pixels = Vec::new();
        for p in 0..(width * width * 4) {
            pixels.push(Pixel {r: 0, g: 0, b: 0, a: 255});
        }

        Cube {
            width,
            anglex: 0,
            angley: 0,
            pixels,
        }
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}
