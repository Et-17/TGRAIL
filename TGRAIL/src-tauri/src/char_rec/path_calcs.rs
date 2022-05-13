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

/// Calcutes direction between two points and returns the
/// Direction enumerable
pub fn calculate_enum_direction(a: &Point, b: &Point) -> Direction {
    let rise = b.y - a.y;
    let run = b.x - a.x;
    match (rise.abs() > run.abs(), rise > 0, run > 0) {
        (true, true, _) => Direction::Up,
        (true, false, _) => Direction::Down,
        (false, _, true) => Direction::Left,
        (false, _, false) => Direction::Right,
    }
}

/// Calculates direction between two points and returns it in degrees
pub fn calculate_direction(a: &Point, b: &Point) -> f32 {
    let rise = (b.y - a.y) as f32;
    let run = (b.x - a.x) as f32;
    match rise.atan2(run) * (180.0 / 3.1415) {
        d if d < 0.0 => d + 360.0,
        d => d,
    }
}

/// Calculates the change between two directions
pub fn calculate_change(a: f32, b: f32) -> f32 {
    match b - a {
        c if c < -180.0 => 360.0 + c,
        c if c > 180.0 => 360.0 - c,
        c => c,
    }
    .abs()
}

/// Determines direction change on a path
pub fn path_enum_directions(path: Vec<Point>) -> Vec<Direction> {
    let mut last = Direction::Still;
    path[1..]
        .iter()
        .zip(path.iter())
        .map(|(x, y)| calculate_enum_direction(&x, &y))
        .filter(|&x| {
            std::mem::discriminant(&last) != {
                last = x;
                std::mem::discriminant(&last)
            }
        })
        .collect()
}

/// Determines the directions of a path in degrees, and returns a tuple
/// with the direction and the corresponding point
pub fn path_directions(path: Vec<Point>) -> Vec<(f32, Point)> {
    path[..]
        .iter()
        .zip(path[1..].iter())
        .map(|(x, y)| calculate_direction(x, y))
        .zip(path[..].iter())
        .map(|(x, &y)| (x, y))
        .collect()
}

/// Finds the corners in a path
pub fn find_corners(path: Vec<(Point)>, sens: f32, corner: f32) -> Vec<Point> {
    let directions = path_directions(path);
    directions[..]
        .iter()
        .zip(directions[1..].iter())
        .zip(directions[2..].iter())
        .zip(directions[3..].iter())
        .zip(directions[4..].iter())
        .filter(|((((a, b), c), d), e)| {
            calculate_change(a.0, b.0) <= sens
                && calculate_change(c.0, e.0) <= sens
                && calculate_change(b.0, c.0) >= corner
        })
        .map(|((((a, b), c), d), e)| c.1)
        .collect()
}
