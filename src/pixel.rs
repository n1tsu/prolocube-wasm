#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct DepthPixel {
    pub depth: f32,
    pub pixel: Pixel,
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

pub const ORANGE_PIXEL: Pixel = Pixel {
    r: 255,
    g: 128,
    b: 0,
    a: 255,
};

pub const YELLOW_PIXEL: Pixel = Pixel {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};
