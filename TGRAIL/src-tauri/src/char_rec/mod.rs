pub use subdefs::Direction;
pub use subdefs::Point;
pub use subdefs::Section;
pub use path_calcs::*;

mod subdefs;
mod path_calcs;

pub struct Character {
    aspect: f32,
    start: Section,
    end: Section,
    corners: Vec<Section>,
    directions: Vec<Direction>,
}