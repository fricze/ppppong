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
