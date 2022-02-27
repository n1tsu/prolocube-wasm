use crate::utils::*;
use euclid::*;

// type Face = (WorldPoint, WorldPoint, WorldPoint, WorldPoint);

pub struct Cube {
    width: f32,
    edges: Vec<WorldPoint>,
    vertices: Vec<Vec<u8>>,
    faces: Vec<(u8, u8, u8, u8)>,
}

impl Cube {
    pub fn new(width: f32, x: f32, y: f32, z: f32) -> Cube {
        let middle = width / 2.0;

        // Construct all edges
        let mut edges = Vec::new();
        for identity in CUBE_IDENTITY_EDGES {
            let point_x = identity.0 * middle + x;
            let point_y = identity.1 * middle + y;
            let point_z = identity.2 * middle + z;
            let point = WorldPoint::new(point_x, point_y, point_z);
            edges.push(point);
        }
        let mut vertices = Vec::new();
        for (i, points) in CUBE_VERTICES_INDEX.iter().enumerate() {
            let mut vec_points = Vec::new();
            for (y, point) in points.iter().enumerate() {
                vec_points.push(*point);
            }
            vertices.push(vec_points);
        }
        let faces = CUBE_FACES.to_vec();
        Cube { width, edges, vertices, faces}
    }

    pub fn rotate(&mut self, roll: f32, pitch: f32, yaw: f32) {
        let angle_roll = Angle::degrees(roll);
        let angle_pitch = Angle::degrees(pitch);
        let angle_yaw = Angle::degrees(yaw);
        let rotation = Rotation3D::<f32, WorldSpace, WorldSpace>::euler(angle_roll, angle_pitch, angle_yaw);

        // I don't find inplace rotation in doc
        let mut rotated_edges = Vec::new();
        for &point in &self.edges {
            let rotated_edge = rotation.transform_point3d(point);
            rotated_edges.push(rotated_edge);
        }
        self.edges = rotated_edges;
    }

    pub fn render(&self, canvas_width: i32, canvas_height: i32) -> Vec<WorldPoint> {
        // let mut pixel_vertices = Vec::new();
        // let mut pixel_faces = Vec::new();
        let mut pixel_edges = Vec::new();

        for &coords in &self.edges {
            pixel_edges.push(coords);
        }

        // remove unvisible faces
        // vec![(depth, index)]
        let mut max_depth_faces = vec![(f32::MIN, 0), (f32::MIN, 0), (f32::MIN, 0)];
        let mut face_index = 0;
        for face in &self.faces {
            let depth = self.edges[face.0 as usize].z +
                self.edges[face.1 as usize].z +
                self.edges[face.2 as usize].z +
                self.edges[face.3 as usize].z;
            for y in 0..3 {
                if depth > max_depth_faces[y].0 {
                    max_depth_faces[y].0 = depth;
                    max_depth_faces[y].1 = face_index;
                    break;
                }
            }
            face_index += 1;
        }
        pixel_edges
    }
}

//     pub fn pixels(&mut self, roll_d: f32, pitch_d: f32, yaw_d: f32) -> *const Pixel {
//         let mut draw_coords = Vec::new();

//         for &coords in &edge_coords {
//             draw_coords.push(coords);
//         }

//         let mut count_pixel = 0.0;
//         for pixel in &mut self.pixels {
//             let a = edge_coords[0];
//             let b = edge_coords[1];
//             let c = edge_coords[3];
//             let d = edge_coords[2];
//             let x = count_pixel % (self.width * 2.0);
//             let y = count_pixel / (self.width * 2.0);
//             if check_inside_rectangle(
//                 a.x + self.width,
//                 a.y + self.width,
//                 b.x + self.width,
//                 b.y + self.width,
//                 c.x + self.width,
//                 c.y + self.width,
//                 d.x + self.width,
//                 d.y + self.width,
//                 x,
//                 y,
//             ) {
//                 *pixel = GREEN_PIXEL;
//             } else {
//                 *pixel = BLACK_PIXEL;
//             }
//             count_pixel += 1.0;
//         }

//         let fbl = vec![1, 2, 4];
//         let fbr = vec![3, 5];
//         let bbl = vec![3, 6];
//         let bbr = vec![7];
//         let ful = vec![5, 6];
//         let fur = vec![7];
//         let bul = vec![7];
//         let mut links = Vec::new();
//         links.push(fbl);
//         links.push(fbr);
//         links.push(bbl);
//         links.push(bbr);
//         links.push(ful);
//         links.push(fur);
//         links.push(bul);

//         let mut count = 0;
//         for list in links {
//             for node in list {
//                 let line = get_lines_coordinates(
//                     edge_coords[count].x,
//                     edge_coords[count].y,
//                     edge_coords[node].x,
//                     edge_coords[node].y,
//                 );
//                 for pixel in line {
//                     draw_coords.push(pixel);
//                 }
//             }
//             count = count + 1;
//         }

//         for coords in &draw_coords {
//             self.pixels
//                 [((coords.y + self.width) * self.width * 2.0 + (coords.x + self.width)) as usize] =
//                 WHITE_PIXEL;
//         }

//         self.pixels.as_ptr()
//     }
