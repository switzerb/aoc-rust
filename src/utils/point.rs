
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point(pub(crate) i32, pub(crate) i32);

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
