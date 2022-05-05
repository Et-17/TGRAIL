pub use subdefs::Direction;
pub use subdefs::Point;

mod subdefs;

pub struct Character {
    aspect: f32,
    start: u8,
    end: u8,
    corners: Vec<u8>,
    directions: Vec<Direction>,
}

/// Using the target point and the point of the low corner (each
/// relative to the upper right point), we convert the target point
/// to one of the 16 segments
pub fn pos_to_seg(corner: Point, target: Point) -> u8 {
    (4 * target.x / corner.x + (4 * (4 * target.y / corner.y)))
        .try_into()
        .unwrap()
}

/// Determines the max borders from the path
/// Returns a vector containing [upper left corner x,
/// upper left corner y, lower right corner x, lower right corner y]
pub fn determine_borders(path: Vec<Point>) -> Vec<Point> {
    let mut up = Point{x: 0, y: 0,};
    let mut down = Point{x: 0, y: 0,};
    for point in path {
        up.x = if point.x < up.x { point.x } else { up.x };
        down.x = if point.x < down.x { point.x } else { up.x };
        up.y = if point.y < up.y { point.y } else { up.x };
        down.y = if point.y < down.y { point.y } else { down.y };
    }
    vec![up, down]
}

/// Calculates the aspect ratio of the bounding rect using the lower
/// right corner, relative to the upper left corner
pub fn calculate_aspect(corner: Point) -> f32 {
    (corner.x as f32) / (corner.y as f32)
}

/// Converts seperate x and y path vectors into a single point vector
pub fn zip_x_y_path(path_x: Vec<i32>, path_y: Vec<i32>) -> Vec<Point> {
    let mut point_path = Vec::<Point>::with_capacity(path_x.len());
    for i in 0..path_x.len() {
        point_path.push(Point {
            x: path_x[i],
            y: path_y[i],
        });
    }
    point_path
}