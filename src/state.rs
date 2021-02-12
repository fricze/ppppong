use ggez::GameResult;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    pub x: f32,
    pub y: f32,
}

pub struct MainState {
    pub horizontal_move: f32,
    pub vertical_move: f32,
    pub ball: Ball,
    pub paddle_left: Paddle,
    pub paddle_right: Paddle,
}

impl MainState {
    pub fn new(x_right: f32) -> GameResult<MainState> {
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

pub fn move_left_paddle(s: &mut MainState, val: f32) {
    s.paddle_left.y = s.paddle_left.y % 800.0 + val;
}

pub fn move_right_paddle(s: &mut MainState, val: f32) {
    s.paddle_right.y = s.paddle_right.y % 800.0 + val;
}
