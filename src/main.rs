use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

#[derive(Debug, Copy, Clone)]
struct Paddle {
    x: f32,
    y: f32,
}

struct MainState {
    pos_x: f32,
    paddle_left: Paddle,
    paddle_right: Paddle,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            paddle_left: Paddle { x: 0.0, y: 0.0 },
            paddle_right: Paddle { x: 100.0, y: 0.0 },
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            2.0,
            graphics::WHITE,
        )?;

        let paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 10.0, 40.0),
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &paddle,
            (Vec2::new(self.paddle_left.x, self.paddle_left.y),),
        )?;

        graphics::draw(
            ctx,
            &paddle,
            (Vec2::new(self.paddle_right.x, self.paddle_right.y),),
        )?;

        graphics::draw(ctx, &circle, (Vec2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Down => {
                self.paddle_left.y = self.paddle_left.y % 800.0 + 4.0;
            }
            KeyCode::Up => {
                self.paddle_left.y = self.paddle_left.y % 800.0 - 4.0;
            }
            _ => (),
        }

        ()
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
