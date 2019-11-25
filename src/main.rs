mod renderer;

use std::error::Error;

use crossterm::input::InputEvent;
use crossterm::input::KeyEvent;

fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = renderer::init(80, 20)?;

    let mut obj =
        renderer::types::Object::from_file(
            "./res/objects/snowflake.yaml")?;

    let mut out = false;
    loop {
        if out {
            break;
        }

        for event in renderer.events() {
            match event {
                InputEvent::Keyboard(KeyEvent::Esc) => {
                    out = true;
                    break;
                }
                InputEvent::Keyboard(KeyEvent::Up) => {
                    obj.y -= 2;
                }
                InputEvent::Keyboard(KeyEvent::Down) => {
                    obj.y += 2;
                }
                InputEvent::Keyboard(KeyEvent::Left) => {
                    obj.x -= 4;
                }
                InputEvent::Keyboard(KeyEvent::Right) => {
                    obj.x += 4;
                }
                _ => {  }
            }
        }

        renderer.render(&vec![&obj])?;
    }

    Ok(())
}
