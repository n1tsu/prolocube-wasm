use euclid::*;

pub struct WorldSpace;
pub type WorldPoint = Point3D<f32, WorldSpace>;

pub fn area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    let value = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0;
    value.abs()
}

pub fn check_inside_rectangle(
    x1f: f32,
    y1f: f32,
    x2f: f32,
    y2f: f32,
    x3f: f32,
    y3f: f32,
    x4f: f32,
    y4f: f32,
    x: f32,
    y: f32,
) -> bool {
    let area_rectangle = area(x1f, y1f, x2f, y2f, x3f, y3f) + area(x1f, y1f, x4f, y4f, x3f, y3f);

    let area1 = area(x, y, x1f, y1f, x2f, y2f);
    let area2 = area(x, y, x2f, y2f, x3f, y3f);
    let area3 = area(x, y, x3f, y3f, x4f, y4f);
    let area4 = area(x, y, x1f, y1f, x4f, y4f);

    if area_rectangle.round() == 0.0 {
        return false;
    }
    area_rectangle.round() + 0.1 > (area1 + area2 + area3 + area4).round()
}

// https://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm#Rust
pub fn get_lines_coordinates(x1f: f32, y1f: f32, x2f: f32, y2f: f32) -> Vec<Point2D<f32, WorldSpace>> {
    let mut coordinates: Vec<Point2D<f32, WorldSpace>> = Vec::new();
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
