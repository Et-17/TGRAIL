use crate::char_rec::subdefs::Direction;
use crate::char_rec::subdefs::Point;
use crate::char_rec::subdefs::Section;

/// Using the target point and the point of the low corner (each
/// relative to the upper right point), we convert the target point
/// to one of the 16 segments
pub fn pos_to_seg(corner: Point, target: Point) -> Section {
    (4 * target.x / corner.x + (4 * (4 * target.y / corner.y)))
        .try_into()
        .unwrap()
}

/// Determines the max borders from the path
/// Returns a vector containing [upper left corner x,
/// upper left corner y, lower right corner x, lower right corner y]
pub fn determine_borders(path: Vec<Point>) -> Vec<Point> {
    let mut up = Point { x: 0, y: 0 };
    let mut down = Point { x: 0, y: 0 };
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

/// Calcutes direction between two points
pub fn calculate_direction(a: &Point, b: &Point) -> Direction {
    let rise = b.y - a.y;
    let run = b.x - a.x;
    match (rise.abs() > run.abs(), rise > 0, run > 0) {
        (true, true, _) => Direction::Up,
        (true, false, _) => Direction::Down,
        (false, _, true) => Direction::Left,
        (false, _, false) => Direction::Right,
    }
}

/// Determines direction change on a path
pub fn path_directions(path: Vec<Point>) -> Vec<Direction> {
    let mut last = Direction::Still;
    path[1..]
        .iter()
        .zip(path.iter())
        .map(|(x, y)| calculate_direction(&x, &y))
        .filter(|&x| {
            std::mem::discriminant(&last) != {
                last = x;
                std::mem::discriminant(&last)
            }
        })
        .collect()
}

/// Calculates the angle between three points in radians, where b is the
/// vertex of the angle
pub fn calculate_angle(a: &Point, b: &Point, c: &Point) -> f64 {
    let y1 = (c.y - b.y) as f64;
    let x1 = (c.x - a.x) as f64;
    let y2 = (b.y - a.y) as f64;
    let x2 = (b.x - a.x) as f64;
    y1.atan2(x1) - y2.atan2(x2)
}
