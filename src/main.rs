use csv;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    input,
    mint::Point2,
    winit::dpi::LogicalSize,
    Context, GameResult,
};
use serde::Deserialize;
use std::io;

struct MainState {
    mouse_down: bool,
    pan: Point2<f32>,
    zoom: f32,
    playing: bool,
    time: u64,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Event {
    tick: u64,
    x: f32,
    y: f32,
}

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

        Ok(MainState {
            mouse_down: false,
            pan: Point2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
            playing: true,
            events,
            time: 0,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.playing {
            self.time += 600;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let mut instance_array = graphics::InstanceArray::new(ctx, None);

        let events = &self.events;
        let zoom_factor = f32::powf(2.0, self.zoom);
        let scale_factor = ctx.gfx.window().scale_factor();

        let mut i = 0;
        while i < events.len() && events[i].tick < self.time {
            let event = &events[i];
            instance_array.push(graphics::DrawParam::new().dest(Vec2::new(
                event.x * scale_factor as f32 * zoom_factor + self.pan.x,
                event.y * scale_factor as f32 * zoom_factor + self.pan.y,
            )));

            i += 1
        }

        canvas.draw(&instance_array, Vec2::new(800.0, 800.0));

        let fps_display =
            graphics::Text::new(format!("FPS: {:.0} Time: {:.0}", ctx.time.fps(), self.time));
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
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == event::MouseButton::Left {
            self.mouse_down = true;
        }

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == event::MouseButton::Left {
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
    event::run(ctx, event_loop, state)
}
