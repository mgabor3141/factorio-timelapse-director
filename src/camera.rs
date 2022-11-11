use ggez::{graphics::Rect, mint::Point2};

use crate::constants::{RESOLUTION, TILE_SIZE_PX};
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
    pub moves: Vec<CameraMove>,
}

pub fn calculate_cameras(events: &Vec<Event>) -> Vec<Camera> {
    let mut cameras = Vec::new();

    for event in events {
        if cameras
            .iter()
            .find(|camera| can_camera_see_event(camera, event))
            .is_none()
        {
            let mut moves = Vec::new();
            moves.push(CameraMove {
                to: CameraPos {
                    // TODO figure out how to use .into() here
                    pos: Point2 {
                        x: event.x,
                        y: event.y,
                    },
                    zoom: 1.0,
                },
                on_tick: event.tick,
            });

            cameras.push(Camera { moves });
        }
    }

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

//

fn can_camera_see_event(camera: &Camera, event: &Event) -> bool {
    let pos = camera_position_on_tick(camera, event.tick);

    if pos.is_none() {
        return false;
    }

    camera_pos_to_rect(&pos.unwrap()).contains(event)
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
