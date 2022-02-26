use euclid::*;
use wasm_bindgen::prelude::*;

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
    points: Vec<Point3D<f32, WorldSpace>>,
}

#[wasm_bindgen]
impl Cube {
    pub fn new(width: f32) -> Cube {
        let mut pixels = Vec::new();
        for _p in 0..(width as u32 * width as u32 * 4) {
            pixels.push(Pixel {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });
        }

        let middle = width / 2.0;

        // line between edges
        // 0 : 1 - 2 - 4
        // 1 : 3 - 5
        // 2 : 3 - 6
        // 3 : 7
        // 4 : 5 - 6
        // 5 : 7
        // 6 : 7
        // 7 :

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

    pub fn pixels(&mut self, roll_d: f32, pitch_d: f32, yaw_d: f32) -> *const Pixel {
        let roll = Angle::degrees(roll_d);
        let pitch = Angle::degrees(pitch_d);
        let yaw = Angle::degrees(yaw_d);
        let rotation = Rotation3D::<f32, WorldSpace, WorldSpace>::euler(roll, pitch, yaw);

        let edge_pixel = Pixel {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let nothing_pixel = Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };

        for pixel in &mut self.pixels {
            *pixel = nothing_pixel;
        }

        let mut edge_coords = Vec::new();
        let mut draw_coords = Vec::new();
        for &point in &self.points {
            // FIXME I need to be smart here for performance
            let rot_point = rotation.transform_point3d(point);
            let coords = rot_point.to_2d().round();
            edge_coords.push(coords);
        }

        for &coords in &edge_coords {
            draw_coords.push(coords);
        }

        let fbl = vec![1, 2, 4];
        let fbr = vec![3, 5];
        let bbl = vec![3, 6];
        let bbr = vec![7];
        let ful = vec![5, 6];
        let fur = vec![7];
        let bul = vec![7];
        let mut links = Vec::new();
        links.push(fbl);
        links.push(fbr);
        links.push(bbl);
        links.push(bbr);
        links.push(ful);
        links.push(fur);
        links.push(bul);

        let mut count = 0;
        for list in links {
            for node in list {
                let line = get_lines_coordinates(
                    edge_coords[count].x,
                    edge_coords[count].y,
                    edge_coords[node].x,
                    edge_coords[node].y,
                );
                for pixel in line {
                    draw_coords.push(pixel);
                }
            }
            count = count + 1;
        }

        for coords in &draw_coords {
            self.pixels
                [((coords.y + self.width) * self.width * 2.0 + (coords.x + self.width)) as usize] =
                edge_pixel;
        }

        self.pixels.as_ptr()
    }
}

// https://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm#Rust
fn get_lines_coordinates(x1f: f32, y1f: f32, x2f: f32, y2f: f32) -> Vec<Point2D<f32, WorldSpace>> {
    let mut coordinates: Vec<Point2D<f32, WorldSpace>> = Vec::new() ;
    let x1 = x1f as i32;
    let y1 = y1f as i32;
    let x2 = x2f as i32;
    let y2 = y2f as i32;
    let dx: i32 = i32::abs(x2 - x1);
    let dy: i32 = i32::abs(y2 - y1);
    let sx: i32 = {
        if x1 < x2 {
            1
        } else {
            -1
        }
    };
    let sy: i32 = {
        if y1 < y2 {
            1
        } else {
            -1
        }
    };

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = x1;
    let mut current_y: i32 = y1;
    loop {
        coordinates.push(Point2D::<f32, WorldSpace>::new(
            current_x as f32,
            current_y as f32,
        ));

        if current_x == x2 && current_y == y2 {
            break;
        }

        let error2: i32 = error;

        if error2 > -dx {
            error -= dy;
            current_x += sx;
        }
        if error2 < dy {
            error += dx;
            current_y += sy;
        }
    }
    coordinates
}
