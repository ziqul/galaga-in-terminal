mod renderer;
mod gameloop;

use std::error::Error;
use std::time::Duration;

use crossterm::input::InputEvent;
use crossterm::input::KeyEvent;

use renderer::types::Renderer;

// screen size
const S_SIZE: (u16, u16) = (80, 20);
const FPS: u8 = 30;

struct GameState {
}

fn main() -> Result<(), Box<dyn Error>> {
    let renderer =
        renderer::init(S_SIZE.0, S_SIZE.1)?;

    let state = GameState {};

    gameloop::gameloop(
        FPS, state, renderer,
        input, update, render)?;

    Ok(())
}

fn input(
    state: &mut GameState,
    inputs: Vec<InputEvent>,
    proceed: &mut bool
) {
    for event in inputs {
        match event {
            InputEvent::Keyboard(KeyEvent::Esc) => {
                *proceed = false;
                break;
            }
            _ => {}
        }
    }
}

fn update(
    state: &mut GameState,
    delta: Duration
) {
}

fn render(
    state: &mut GameState,
    renderer: &Renderer
) ->
    Result<(), Box<dyn Error>>
{
    Ok(())
}
