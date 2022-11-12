use geo_types::{coord, Rect};
use ggez::graphics;

pub fn rect_to_geo(rect: &graphics::Rect) -> Rect {
    let graphics::Rect { x, y, w, h } = *rect;

    Rect::new(
        coord! { x: x as f64 , y: y as f64  },
        coord! { x: (x + w) as f64 , y: (y + h) as f64 },
    )
}
