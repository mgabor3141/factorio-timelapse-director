mod camera;
mod constants;
mod conversions;
mod event;
mod state;

use csv;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{run, EventHandler, MouseButton},
    glam::*,
    graphics::{self, Color, DrawMode, StrokeOptions},
    input,
    mint::Point2,
    winit::dpi::LogicalSize,
    Context, GameResult,
};
use state::{MainState, WhatToDraw};
use std::io;

use camera::*;
use event::Event;

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut events = Vec::new();

        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.deserialize() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let event: Event = match result {
                Ok(r) => r,
                Err(_) => panic!("Error loading CSV"),
            };

            events.push(event)
        }

        let cameras = calculate_cameras(&events);

        Ok(MainState {
            mouse_down: false,
            pan: Point2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
            playing: true,
            playback_speed: 16,
            time: 0,
            events,
            cameras,
            what_to_draw: WhatToDraw::default(),
        })
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.playing {
            self.time += self.playback_speed as u64 * ctx.time.delta().as_millis() as u64;
        }

        if self.time > self.events.last().unwrap().tick {
            self.time = self.events.last().unwrap().tick;
            self.playing = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw point cloud
        let mut instance_array = graphics::InstanceArray::new(ctx, None);

        let events = &self.events;
        let scale_factor = ctx.gfx.window().scale_factor();
        let zoom_factor = f32::powf(2.0, self.zoom) * scale_factor as f32;

        for event in events {
            if event.tick > self.time {
                break;
            }

            instance_array.push(graphics::DrawParam::new().dest(Vec2::new(
                event.x * zoom_factor + self.pan.x,
                event.y * zoom_factor + self.pan.y,
            )));
        }

        canvas.draw(&instance_array, Vec2::new(800.0, 800.0));

        // Draw camera rectangles
        if self.what_to_draw.camera_rectangles {
            for cam in &self.cameras {
                let rect_option = camera_to_rect(cam, self.time);
                if rect_option.is_none() {
                    continue;
                }

                let rect = graphics::Rect {
                    x: rect_option.unwrap().x * zoom_factor + self.pan.x,
                    y: rect_option.unwrap().y * zoom_factor + self.pan.y,
                    w: rect_option.unwrap().w * zoom_factor,
                    h: rect_option.unwrap().h * zoom_factor,
                };

                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        ctx,
                        DrawMode::Stroke(StrokeOptions::default()),
                        rect,
                        graphics::Color::from([0.4, 0.3, 0.2, 1.0]),
                    )
                    .unwrap(),
                    Vec2::new(800.0, 800.0),
                )
            }
        }

        // Draw text
        let fps_display = graphics::Text::new(format!(
            "FPS: {:.0} Time: {:02}:{:02} Playback Speed: {}x",
            ctx.time.fps(),
            self.time / (60 * 60 * 60),
            self.time / (60 * 60) % 60,
            if self.playing { self.playback_speed } else { 0 },
        ));
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([10.0, 10.0]).color(Color::WHITE),
        );

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        self.zoom += y / 20.0;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.mouse_down = true;
        }

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.mouse_down = false;
        }

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        if self.mouse_down {
            self.pan = Point2 {
                x: self.pan.x + dx,
                y: self.pan.y + dy,
            };
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(input::keyboard::KeyCode::Space) => self.playing = !self.playing,
            Some(input::keyboard::KeyCode::R) => self.time = 0,
            Some(input::keyboard::KeyCode::LBracket) => self.playback_speed /= 2,
            Some(input::keyboard::KeyCode::RBracket) => self.playback_speed *= 2,
            Some(input::keyboard::KeyCode::C) => {
                self.what_to_draw.camera_rectangles = !self.what_to_draw.camera_rectangles
            }
            Some(input::keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => (),
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("points-framer", "ggez")
        .window_setup(WindowSetup::default().title("Factorio Replay Events Points Scatter"))
        .window_mode(WindowMode {
            logical_size: Some(LogicalSize::new(800.0, 600.0)),
            resizable: true,
            ..Default::default()
        });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    run(ctx, event_loop, state)
}
