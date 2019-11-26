mod renderer;

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crossterm::input::InputEvent;
use crossterm::input::KeyEvent;
use rand::Rng;

use renderer::types::Location;
use renderer::types::Renderer;
use renderer::types::Representation;

struct Snflk<'a> {
    // cells per second
    cps: i32,
    x: f32,
    y: f32,
    lctn: Location,
    repr: &'a Representation
}

fn main() -> Result<(), Box<dyn Error>> {
    let screen_size = (80, 20);
    let fps = 30;

    let mut renderer =
        renderer::init(screen_size.0, screen_size.1)?;

    let obj_r =
        Representation::from_file(
            "./res/objects/snowflake2.yaml")?;

    let mut objects = Vec::<Snflk>::new();
    for i in 0..50 {
        objects.push(create(screen_size.0, &obj_r));
    }

    let mut start = Instant::now();
    let mut out = false;
    loop {

        if out { break; }

        for event in renderer.events() {
            match event {
                InputEvent::Keyboard(KeyEvent::Esc) => {
                    out = true;
                    break;
                }
                _ => {  }
            }
        }

        tick(start.elapsed(), &mut objects);

        rndr(&mut renderer, &objects);

        let duration = start.elapsed();
        let wait_time =
            Duration::from_millis(1000 / fps);

        if duration.as_millis() < wait_time.as_millis() {
            thread::sleep(wait_time - duration);
        }

        start = Instant::now();
    }

    Ok(())
}

fn create<'a>(max_w: u16, repr: &'a Representation) -> Snflk<'a> {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0, max_w as i64);
    let y = rng.gen_range(0, 20);

    Snflk {
        x: x as f32,
        y: y as f32,
        cps: rng.gen_range(15, 20),
        repr: repr,
        lctn: Location {
            x: x,
            y: y,
        }
    }
}

fn rndr(renderer: &mut Renderer, objects: &Vec<Snflk>) {

    let mut for_render =
        Vec::<(&Location, &Representation)>::new();

    for o in objects {
        for_render.push((&o.lctn, &o.repr));
    }

    renderer.render(&for_render);
}

fn tick(delta: Duration, objects: &mut Vec<Snflk>) {
    for o in objects {
        o.y += o.cps as f32 * delta.as_secs_f32() * 100.0;
        o.lctn.y = o.y.floor() as i64;

        if o.lctn.y >= 20 {
            o.lctn.y = 0;
            o.y = 0.0;
        }
    }
}
