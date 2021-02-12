use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

mod draw;
mod events;
mod state;

impl event::EventHandler for state::MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let rect = ggez::graphics::screen_coordinates(&ctx);

        let right_wall_collision = self.ball.x + 5.0 >= rect.w;
        let left_wall_collision = self.ball.x <= 5.0;

        let left_paddle_collision =
            self.ball.y <= self.paddle_left.y + 40.0 && self.ball.y >= self.paddle_left.y - 10.0;
        let right_paddle_collision =
            self.ball.y <= self.paddle_right.y + 40.0 && self.ball.y >= self.paddle_right.y - 10.0;

        let right_paddle_distance = self.ball.y - self.paddle_right.y - 18.0;
        let left_paddle_distance = self.ball.y - self.paddle_left.y - 18.0;

        if right_wall_collision && right_paddle_collision {
            self.horizontal_move = -3.0;
            self.vertical_move = right_paddle_distance / 10.0;
        } else if left_wall_collision && left_paddle_collision {
            self.horizontal_move = 3.0;
            self.vertical_move = left_paddle_distance / 10.0;
        }

        self.ball.x = self.ball.x % 800.0 + self.horizontal_move;
        self.ball.y = self.ball.y % 800.0 + self.vertical_move;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        draw::ball(ctx, &self.ball)?;
        draw::paddle(ctx, &self.paddle_left)?;
        draw::paddle(ctx, &self.paddle_right)?;

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
        events::key_down(keycode, self);
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let rect = ggez::graphics::screen_coordinates(&ctx);
    let state = state::MainState::new(rect.w - 10.0)?;
    event::run(ctx, event_loop, state)
}
