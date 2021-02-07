use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

struct Paddle {
    x: f32,
    y: f32,
}

struct MainState {
    pos_x: f32,
    move_val: f32,
    paddle_left: Paddle,
    paddle_right: Paddle,
}

impl MainState {
    fn new(x_right: f32) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            paddle_left: Paddle { x: 0.0, y: 0.0 },
            paddle_right: Paddle { x: x_right, y: 0.0 },
            move_val: 3.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let rect = ggez::graphics::screen_coordinates(&ctx);

        if self.pos_x + 5.0 >= rect.w {
            self.move_val = -3.0;
        } else if self.pos_x <= 0.0 {
            self.move_val = 3.0;
        }

        self.pos_x = self.pos_x % 800.0 + self.move_val;

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
                self.paddle_left.y = self.paddle_left.y % 800.0 + 12.0;
            }
            KeyCode::Up => {
                self.paddle_left.y = self.paddle_left.y % 800.0 - 12.0;
            }
            KeyCode::J => {
                self.paddle_right.y = self.paddle_right.y % 800.0 + 12.0;
            }
            KeyCode::K => {
                self.paddle_right.y = self.paddle_right.y % 800.0 - 12.0;
            }
            _ => (),
        }

        ()
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let rect = ggez::graphics::screen_coordinates(&ctx);
    let state = MainState::new(rect.w - 10.0)?;
    event::run(ctx, event_loop, state)
}
