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
        &mut self, objects: &Vec<&types::Object>
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
            let owidth = (*o).data.len();
            let oheight = (*o).data[0].len();

            for i in 0..owidth {
                for j in 0..oheight {
                    // null_char
                    let nc = (*o).null_char;

                    if (*o).data[i][j] != nc {
                        // positions relative to frame
                        // "frame_x", "frame_y"
                        let fx = (*o).x + j;
                        let fy = (*o).y + i;

                        new_frame[fx][fy] =
                            (*o).data[i][j];
                    }
                }
            }
        }

        for x in 0..fwidth {
            for y in 0..fheight {
                if (*self).old_frame != new_frame {
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
