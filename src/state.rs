use ggez::mint::Point2;

use crate::{camera::Camera, event::Event};

pub struct WhatToDraw {
    pub camera_rectangles: bool,
}

impl Default for WhatToDraw {
    fn default() -> WhatToDraw {
        WhatToDraw {
            camera_rectangles: true,
        }
    }
}

pub struct MainState {
    pub mouse_down: bool,
    pub pan: Point2<f32>,
    pub zoom: f32,
    pub playing: bool,
    pub playback_speed: u32,
    pub time: u64,
    pub events: Vec<Event>,
    pub cameras: Vec<Camera>,
    pub what_to_draw: WhatToDraw,
}
