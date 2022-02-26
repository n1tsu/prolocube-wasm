use wasm_bindgen::prelude::*;
use euclid::*;

extern crate web_sys;

pub struct ScreenSpace;
pub type ScreenPoint = Point2D<f32, ScreenSpace>;
pub type ScreenSize = Size2D<f32, ScreenSpace>;
pub struct WorldSpace;
pub type WorldPoint = Point3D<f32, WorldSpace>;
pub type ProjectionMatrix = Transform3D<f32, WorldSpace, ScreenSpace>;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
    width: f32,
    pixels: Vec<Pixel>,
    points: Vec<Point3D::<f32, WorldSpace>>,
}

#[wasm_bindgen]
impl Cube {
    pub fn new(width: f32) -> Cube {
        let mut pixels = Vec::new();
        for _p in 0..(width as u32 * width as u32 * 4) {
            pixels.push(Pixel {r: 0, g: 0, b: 0, a: 255});
        }

        let middle = width / 2.0;

        let front_bottom_left = Point3D::<f32, WorldSpace>::new(-middle, -middle, -middle);
        let front_bottom_right = Point3D::<f32, WorldSpace>::new(middle, -middle, -middle);

        let back_bottom_left = Point3D::<f32, WorldSpace>::new(-middle, middle, -middle);
        let back_bottom_right = Point3D::<f32, WorldSpace>::new(middle, middle, -middle);

        let front_up_left = Point3D::<f32, WorldSpace>::new(-middle, -middle, middle);
        let front_up_right = Point3D::<f32, WorldSpace>::new(middle, -middle, middle);

        let back_up_left = Point3D::<f32, WorldSpace>::new(-middle, middle, middle);
        let back_up_right = Point3D::<f32, WorldSpace>::new(middle, middle, middle);


        let mut points = Vec::new();
        points.push(front_bottom_left);
        points.push(front_bottom_right);
        points.push(back_bottom_left);
        points.push(back_bottom_right);
        points.push(front_up_left);
        points.push(front_up_right);
        points.push(back_up_left);
        points.push(back_up_right);

        Cube {
            width,
            pixels,
            points,
        }
    }

    pub fn pixels(&mut self, roll_d: f32, pitch_d: f32 , yaw_d: f32) -> *const Pixel {
        let roll = Angle::degrees(roll_d);
        let pitch = Angle::degrees(pitch_d);
        let yaw = Angle::degrees(yaw_d);
        let rotation = Rotation3D::<f32, WorldSpace, WorldSpace>::euler(roll, pitch, yaw);

        let edge_pixel = Pixel {r: 255, g: 255, b: 255, a: 255};
        let nothing_pixel = Pixel {r: 0, g: 0, b: 0, a: 255};

        for pixel in &mut self.pixels {
            *pixel = nothing_pixel;
        }

        for point in &self.points {
            // FIXME I need to be smart here for performance
            let rot_point = rotation.transform_point3d(point.clone());

            let coords = rot_point.to_2d().round();
            self.pixels[((coords.y + self.width) * self.width * 2.0 + (coords.x + self.width)) as usize] = edge_pixel;

            log!("{:?} -> {:?} : {:?}", rot_point, rot_point.to_2d().round(), (coords.y * self.width + coords.x) as usize);
        }

        self.pixels.as_ptr()
    }
}

fn drawline(Point2D origin, Point2D endpoint) -> Vec<Point2D>{
    let deltaX = endpoint.x - origin.x;
    let deltaY = endpoint.y - origin.y;
    let error = 0.0;
    let mut result = Vec::new();

    // Note the below fails for completely vertical lines.
    let deltaError = 0.0;
    if (deltaY != 0.0) {
        deltaError = deltaY / deltaX;
    }

    let y = origin.y as u32;
    for x in (origin.x as u32)..(endpoint.x as u32) {
        result.push(Point2D::new(x, y))
        error = error + deltaError
        if (error >= 0.5) {
            y += 1;
            error -= 1.0;
        }
    }
}
