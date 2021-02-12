use crate::state;
use crate::state::MainState;
use ggez::event::KeyCode;

pub fn key_down(keycode: KeyCode, state: &mut MainState) {
    match keycode {
        KeyCode::Down => {
            state::move_left_paddle(state, 12.0);
        }
        KeyCode::Up => {
            state::move_left_paddle(state, -12.0);
        }
        KeyCode::J => {
            state::move_right_paddle(state, 12.0);
        }
        KeyCode::K => {
            state::move_right_paddle(state, -12.0);
        }
        _ => (),
    }

    ()
}
