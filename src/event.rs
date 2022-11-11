use ggez::mint::Point2;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub tick: u64,
    pub x: f32,
    pub y: f32,
}

impl Into<Point2<f32>> for Event {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Point2<f32>> for &Event {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}
