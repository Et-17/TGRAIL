pub struct Character {
    aspect: f32,
    start: u8,
    end: u8,
    corners: Vec<u8>,
    directions: Vec<u8>,
}

impl Character {
    /// Using the target point and the point of the low corner (each
    /// relative to the upper right point), we convert the target point
    /// to one of the 16 segments
    pub fn pos_to_seg(xc: i32, yc: i32, xt: i32, yt: i32) -> i8 {
        (((4 * xt) / xc) + (4 * ((4 * yt) / yc)))
            .try_into()
            .unwrap()
    }

    /// Determines the max borders from the path
    /// Returns a vector containing [upper left corner x,
    /// upper left corner y, lower right corner x, lower right corner y]
    pub fn determine_borders(path_x: Vec<i32>, path_y: Vec<i32>) -> Vec<i32> {
        let mut up = vec![0i32; 2];
        let mut down = vec![0i32; 2];
        for point in path_x {
            up[0] = if point < up[0] { point } else { up[0] };
            down[0] = if point > down[0] { point } else { down[0] };
        }
        for point in path_y {
            up[1] = if point < up[1] { point } else { up[0] };
            down[1] = if point > down[1] { point } else { down[1] };
        }
        vec![up[0], up[1], down[0], down[1]]
    }

    /// Calculates the aspect ratio of the bounding rect using the lower
    /// right corner, relative to the upper left corner
    pub fn calculate_aspect(xc: i32, yc: i32) -> f32 {
        (xc as f32) / (yc as f32)
    }
}
