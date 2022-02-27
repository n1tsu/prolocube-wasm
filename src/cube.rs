use euclid::*;
use wasm_bindgen::prelude::*;
use crate::utils::*;
use crate::render::*;

type Face = (WorldPoint, WorldPoint, WorldPoint, WorldPoint);

#[wasm_bindgen]
pub struct Cube {
    width: f32,
    edges: Vec<WorldPoint>,
    // vertices: Vec<Vec<WorldPoint>>, // Adjacency list
    // faces: Vec<Face>,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Cube {
    pub fn new(width: f32) -> Cube {
        let mut pixels = Vec::new();
        for _p in 0..(width as u32 * width as u32 * 4) {
            pixels.push(BLACK_PIXEL);
        }

        let middle = width / 2.0;

        let front_bottom_left = WorldPoint::new(-middle, -middle, -middle);
        let front_bottom_right = WorldPoint::new(middle, -middle, -middle);

        let back_bottom_left = WorldPoint::new(-middle, middle, -middle);
        let back_bottom_right = WorldPoint::new(middle, middle, -middle);

        let front_up_left = WorldPoint::new(-middle, -middle, middle);
        let front_up_right = WorldPoint::new(middle, -middle, middle);

        let back_up_left = WorldPoint::new(-middle, middle, middle);
        let back_up_right = WorldPoint::new(middle, middle, middle);

        let mut edges = Vec::new();
        edges.push(front_bottom_left);
        edges.push(front_bottom_right);
        edges.push(back_bottom_left);
        edges.push(back_bottom_right);
        edges.push(front_up_left);
        edges.push(front_up_right);
        edges.push(back_up_left);
        edges.push(back_up_right);

        Cube {
            width,
            pixels,
            edges,
        }
    }

    pub fn pixels(&mut self, roll_d: f32, pitch_d: f32, yaw_d: f32) -> *const Pixel {
        let roll = Angle::degrees(roll_d);
        let pitch = Angle::degrees(pitch_d);
        let yaw = Angle::degrees(yaw_d);
        let rotation = Rotation3D::<f32, WorldSpace, WorldSpace>::euler(roll, pitch, yaw);

        let mut edge_coords = Vec::new();
        let mut draw_coords = Vec::new();
        for &point in &self.edges {
            // FIXME I need to be smart here for performance
            let rot_point = rotation.transform_point3d(point);
            let coords = rot_point.to_2d().round();
            edge_coords.push(coords);
        }

        for &coords in &edge_coords {
            draw_coords.push(coords);
        }
        let mut count_pixel = 0.0;
        for pixel in &mut self.pixels {
            let a = edge_coords[0];
            let b = edge_coords[1];
            let c = edge_coords[3];
            let d = edge_coords[2];
            let x = count_pixel % (self.width * 2.0);
            let y = count_pixel / (self.width * 2.0);
            if check_inside_rectangle(
                a.x + self.width,
                a.y + self.width,
                b.x + self.width,
                b.y + self.width,
                c.x + self.width,
                c.y + self.width,
                d.x + self.width,
                d.y + self.width,
                x,
                y,
            ) {
                *pixel = GREEN_PIXEL;
            } else {
                *pixel = BLACK_PIXEL;
            }
            count_pixel += 1.0;
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
                WHITE_PIXEL;
        }

        self.pixels.as_ptr()
    }
}
