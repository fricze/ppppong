use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

struct Paddle {
    x: f32,
    y: f32,
}

struct Ball {
    x: f32,
    y: f32,
}

struct MainState {
    horizontal_move: f32,
    vertical_move: f32,
    ball: Ball,
    paddle_left: Paddle,
    paddle_right: Paddle,
}

impl MainState {
    fn new(x_right: f32) -> GameResult<MainState> {
        let s = MainState {
            paddle_left: Paddle { x: 0.0, y: 0.0 },
            paddle_right: Paddle { x: x_right, y: 0.0 },
            ball: Ball { x: 0.0, y: 300.0 },
            horizontal_move: 3.0,
            vertical_move: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
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

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            1.0,
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

        graphics::draw(ctx, &circle, (Vec2::new(self.ball.x, self.ball.y),))?;

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
