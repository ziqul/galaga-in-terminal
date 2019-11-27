use std::error::Error;
use std::time::Duration;
use std::time::Instant;
use std::thread;

use crossterm::input::InputEvent;

use crate::renderer::types::Renderer;

pub fn gameloop<T>(
    fps: u8,
    mut state: T,
    mut renderer: Renderer,
    input: fn(&mut T, Vec<InputEvent>, &mut bool) -> (),
    update: fn(&mut T, Duration) -> (),
    render: fn(&mut T, &mut Renderer) -> Result<(), Box<dyn Error>>,
) ->
    Result<(), Box<dyn Error>>
{
    let timer = Instant::now();

    let pref_loop_dur = Duration::from_millis(1000 / fps as u64);
    let mut curr_loop_start;
    let mut prev_loop_start = timer.elapsed();

    let mut proceed = true;
    while proceed {
        curr_loop_start = timer.elapsed();
        let prev_loop_dur = curr_loop_start - prev_loop_start;
        prev_loop_start = curr_loop_start;

        input(
            &mut state,
            renderer.events(),
            &mut proceed);

        update(
            &mut state,
            prev_loop_dur);

        render(
            &mut state,
            &mut renderer)?;

        let curr_loop_end = timer.elapsed();
        let curr_loop_dur = curr_loop_end - curr_loop_start;

        if pref_loop_dur > curr_loop_dur {
            thread::sleep(pref_loop_dur - curr_loop_dur);
        }
    }

    Ok(())
}
