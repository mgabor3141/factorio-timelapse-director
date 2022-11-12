use std::time::Instant;

use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::Point;
use ggez::{graphics::Rect, mint::Point2};

use crate::constants::*;
use crate::conversions::rect_to_geo;
use crate::event::Event;

#[derive(Debug)]
pub struct CameraPos {
    pub pos: Point2<f32>,
    pub zoom: f32,
}

#[derive(Debug)]
pub struct CameraMove {
    pub to: CameraPos,
    pub on_tick: u64,
}

#[derive(Debug)]
pub struct Camera {
    moves: Vec<CameraMove>,
}

impl Camera {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn add_move(&mut self, m: CameraMove) -> () {
        self.moves.push(m)
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
            to: CameraPos {
                pos: event.into(),
                zoom: CAMERA_TARGET_MAXIMUM_ZOOM,
            },
            on_tick: event.tick,
        });
    }

    println!("Done! Took {}s.", start_time.elapsed().as_secs());

    cameras
}

pub fn camera_to_rect(camera: &Camera, tick: u64) -> Option<Rect> {
    let pos = camera_position_on_tick(camera, tick);

    if pos.is_none() {
        return None;
    }

    Some(camera_pos_to_rect(&pos.unwrap()))
}

fn camera_pos_to_rect(camera: &CameraPos) -> Rect {
    let w = RESOLUTION.x as f32 / TILE_SIZE_PX as f32;
    let h = RESOLUTION.y as f32 / TILE_SIZE_PX as f32;

    Rect {
        x: camera.pos.x - w / 2.0,
        y: camera.pos.y - h / 2.0,
        w,
        h,
    }
}

fn camera_event_distance(camera: &Camera, event: &Event) -> f64 {
    let pos = camera_position_on_tick(camera, event.tick);

    if pos.is_none() {
        return f64::INFINITY;
    }

    let camera_rect = camera_pos_to_rect(&pos.unwrap());
    let camera_geo = rect_to_geo(&camera_rect);

    camera_geo
        .to_polygon()
        .euclidean_distance(&Point::new(event.x as f64, event.y as f64))
}

fn camera_position_on_tick(camera: &Camera, tick: u64) -> Option<&CameraPos> {
    let mut pos = None;

    for camera_move in &camera.moves {
        if camera_move.on_tick == tick {
            pos = Some(&camera_move.to);
            break;
        } else if camera_move.on_tick > tick {
            break;
        }
        pos = Some(&camera_move.to);
    }

    pos
}
