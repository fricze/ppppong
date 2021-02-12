use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

use crate::state::{Ball, Paddle};

pub fn paddle(ctx: &mut Context, paddle: &Paddle) -> GameResult {
    let paddle_graphic = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 10.0, 40.0),
        graphics::WHITE,
    )?;

    graphics::draw(ctx, &paddle_graphic, (Vec2::new(paddle.x, paddle.y),))?;

    Ok(())
}

pub fn ball(ctx: &mut Context, ball: &Ball) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        10.0,
        1.0,
        graphics::WHITE,
    )?;

    graphics::draw(ctx, &circle, (Vec2::new(ball.x, ball.y),))?;

    Ok(())
}
