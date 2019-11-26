use std::error::Error;
use std::io::Stdout;
use std::io::Write;

use crossterm::Output;
use crossterm::QueueableCommand;
use crossterm::cursor;
use crossterm::input::AsyncReader;
use crossterm::input::InputEvent;
use crossterm::screen::RawScreen;

use crate::renderer::types;

pub struct Renderer {
    _raw: RawScreen,
    stdout: Stdout,
    old_frame: Vec<Vec<char>>,
    reader: AsyncReader,
}

impl Renderer {
    pub fn new(
        raw: RawScreen, reader: AsyncReader,
        win_width: u16, win_height: u16,
        stdout: Stdout
    ) -> Renderer {
        let width = win_width as usize;
        let height = win_height as usize;

        Renderer {
            _raw: raw,
            reader: reader,
            stdout: stdout,
            old_frame: vec![vec![' '; height]; width],
        }
    }

    pub fn render(
        &mut self,
        objects: &Vec<(&types::Location, &types::Representation)>
    ) ->
        Result<(), Box<dyn Error>>
    {
        // frame width, frame height
        let fwidth = (*self).old_frame.len();
        let fheight = (*self).old_frame[0].len();

        let mut new_frame =
            vec![vec![' '; fheight]; fwidth];

        for o in objects {
            // object width, object height
            let owidth = o.1.data().len();
            let oheight = o.1.data()[0].len();

            for i in 0..owidth {
                for j in 0..oheight {
                    // null_char
                    let nc = o.1.null_char();

                    // positions relative to frame
                    // "frame_x", "frame_y"
                    let fx = o.0.x + j as i32;
                    let fy = o.0.y + i as i32;

                    if
                        o.1.data()[i][j] != nc &&
                        fx >= 0 && fx < fwidth as i32 &&
                        fy >= 0 && fy < fheight as i32
                    {
                        let fx_u = fx as usize;
                        let fy_u = fy as usize;

                        new_frame[fx_u][fy_u] =
                            o.1.data()[i][j];
                    }
                }
            }
        }

        for x in 0..fwidth {
            for y in 0..fheight {
                if (*self).old_frame[x][y] != new_frame[x][y] {
                    (*self).stdout.queue(
                        cursor::MoveTo(
                            x as u16, y as u16)
                    )?;

                    (*self).stdout.queue(
                        Output(new_frame[x][y])
                    )?;
                }
            }
        }

        (*self).stdout.flush()?;
        (*self).old_frame = new_frame;

        Ok(())
    }

    pub fn events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        while let Some(event) = (*self).reader.next() {
            events.push(event);
        }

        events
    }
}
