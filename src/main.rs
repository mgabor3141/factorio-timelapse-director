use ggez::{
    event,
    glam::*,
    graphics::{self},
    Context, GameResult
};
use ggez::conf::{WindowMode, WindowSetup};
use std::io;
use csv;
use serde::Deserialize;

struct MainState {
    instance_array: graphics::InstanceArray,
}

#[derive(Debug, Deserialize)]
struct Event {
    tick: u64,
    x: f32,
    y: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut instance_array = graphics::InstanceArray::new(
            ctx,
            None
        );

        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.deserialize() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let event: Event = match result {
                Ok(r) => r,
                Err(_) => panic!("Error loading CSV")
            };

            instance_array.push(graphics::DrawParam::new()
                .dest(Vec2::new(event.x, event.y))
            );
        }    

        Ok(MainState { instance_array })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.instance_array, Vec2::new(800.0, 800.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("points-framer", "ggez")
        .window_setup(WindowSetup::default().title("Factorio Replay Events Points Scatter"))
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(true),
        );
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
