pub mod types;

use std::error::Error;
use std::io::stdout;

use crossterm::ExecutableCommand;
use crossterm::input::input;
use crossterm::screen::RawScreen;
use crossterm::terminal;
use crossterm::cursor;

pub fn init(
    win_width: u16, win_height: u16
) ->
    Result<types::Renderer, Box<dyn Error>>
{
    let raw = RawScreen::into_raw_mode()?;
    let reader = input().read_async();

    let mut stdout = stdout();

    stdout.execute(
        terminal::SetSize(
            win_width, win_height))?;
    stdout.execute(
        cursor::Hide)?;

    Ok(types::Renderer::new(
        raw, reader, win_width, win_height, stdout))
}
