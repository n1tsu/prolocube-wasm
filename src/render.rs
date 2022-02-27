#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub const BLACK_PIXEL: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const WHITE_PIXEL: Pixel = Pixel {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const RED_PIXEL: Pixel = Pixel {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

pub const BLUE_PIXEL: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

pub const GREEN_PIXEL: Pixel = Pixel {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
