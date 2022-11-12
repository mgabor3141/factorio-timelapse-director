use geo::Point;
use ggez::mint::Point2;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawEvent {
    pub tick: u64,
    pub x: f32,
    pub y: f32,
}

pub struct Event {
    pub tick: u64,
    pub x: f32,
    pub y: f32,
    pub point: Point2<f32>,
    pub geo_point: Point,
}

impl Event {
    pub fn new(e: RawEvent) -> Self {
        Self {
            tick: e.tick,
            x: e.x,
            y: e.y,
            point: Point2 { x: e.x, y: e.y },
            geo_point: Point::new(e.x as f64, e.y as f64),
        }
    }
}
