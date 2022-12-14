use std::collections::HashMap;
use std::time::Instant;

use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo_types;
use ggez::{graphics::Rect, mint::Point2};

use crate::constants::*;
use crate::event::Event;

#[derive(Debug)]
pub struct CameraPos {
    pub pos: Point2<f32>,
    pub zoom: f32,

    rect: Rect,
    polygon: geo_types::Polygon,
}

impl CameraPos {
    fn new(pos: Point2<f32>, zoom: f32) -> Self {
        let w = RESOLUTION.x as f32 / TILE_SIZE_PX as f32;
        let h = RESOLUTION.y as f32 / TILE_SIZE_PX as f32;

        let rect = Rect {
            x: pos.x - w / 2.0,
            y: pos.y - h / 2.0,
            w,
            h,
        };

        Self {
            pos,
            zoom,
            rect,
            polygon: geo_types::Rect::new(
                geo_types::coord! { x: rect.x as f64 , y: rect.y as f64  },
                geo_types::coord! { x: (rect.x + w) as f64 , y: (rect.y + h) as f64 },
            )
            .to_polygon(),
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
}

#[derive(Debug)]
pub struct CameraMove {
    pub to: CameraPos,
    pub on_tick: u64,
}

#[derive(Debug)]
pub struct Camera {
    moves: Vec<CameraMove>,
    position_cache: HashMap<u64, CameraPos>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            position_cache: HashMap::new(),
        }
    }

    pub fn add_move(&mut self, m: CameraMove) -> () {
        self.moves.push(m)
    }

    pub fn position(&self, tick: u64) -> Option<&CameraPos> {
        match self.position_cache.get(&tick) {
            None => {
                for camera_move in self.moves.iter().rev() {
                    if camera_move.on_tick <= tick {
                        return Some(&camera_move.to);
                    }
                }

                None
            }
            Some(p) => Some(p),
        }
    }
}

pub fn calculate_cameras(events: &Vec<Event>) -> Vec<Camera> {
    let start_time = Instant::now();
    println!("Calculating camera moves...");

    let mut cameras = Vec::new();

    for event in events {
        let closest_camera = cameras.iter_mut().min_by(|camera, other| {
            camera_event_distance(camera, event).total_cmp(&camera_event_distance(other, event))
        });

        let camera = match closest_camera {
            None => {
                cameras.push(Camera::new());
                cameras.last_mut().unwrap()
            }
            Some(camera) => {
                if camera_event_distance(camera, event) > CAMERA_FOLLOW_THRESHOLD {
                    cameras.push(Camera::new());
                    cameras.last_mut().unwrap()
                } else {
                    camera
                }
            }
        };

        camera.add_move(CameraMove {
            to: CameraPos::new(event.point, CAMERA_TARGET_MAXIMUM_ZOOM),
            on_tick: event.tick,
        });
    }

    println!("Done! Took {}s.", start_time.elapsed().as_secs());

    cameras
}

fn camera_event_distance(camera: &Camera, event: &Event) -> f64 {
    let pos = camera.position(event.tick);

    match pos {
        None => f64::INFINITY,
        Some(pos) => pos.polygon.euclidean_distance(&event.geo_point),
    }
}
