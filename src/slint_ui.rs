use yapt::Point;

slint::include_modules!();

impl SlintPoint {
    pub fn into_point(self) -> Point<u32> {
        Point {
            x: self.x.unsigned_abs(),
            y: self.y.unsigned_abs(),
        }
    }
}
