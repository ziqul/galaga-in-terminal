mod renderer;
mod gameloop;

use std::error::Error;
use std::time::Duration;

use crossterm::input::InputEvent;
use crossterm::input::KeyEvent;

use renderer::types::Location;
use renderer::types::Renderer;
use renderer::types::Representation;

// screen size
const S_SIZE: (u16, u16) = (100, 30);
const FPS: u8 = 30;

struct Bullet {
    pub xf: f32,
    pub yf: f32,
    pub x: i32,
    pub y: i32,
    pub speed: i32
}

struct Turret {
    pub x: i32,
    pub y: i32,
    pub view: usize,
    pub speed: i32
}

struct GameState {
    views: Vec<Representation>,
    turret: Turret,
    bullet_view: usize,
    bullets: Vec<Bullet>
}

fn main() -> Result<(), Box<dyn Error>> {
    let renderer =
        renderer::init(S_SIZE.0, S_SIZE.1)?;

    let mut views = Vec::<Representation>::new();

    let turret_view =
        load_view(
            "./res/objects/turret.yaml", &mut views)?;
    let bullet_view =
        load_view(
            "./res/objects/bullet.yaml", &mut views)?;

    let state = GameState {
        views: views,
        turret: Turret {
            speed: 4,
            x: S_SIZE.0 as i32 / 2 - 5,
            y: S_SIZE.1 as i32 - 2,
            view: turret_view
        },
        bullet_view: bullet_view,
        bullets: Vec::<Bullet>::new()
    };

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
    // *proceed = false;

    for event in inputs {
        match event {
            InputEvent::Keyboard(KeyEvent::Esc) => {
                *proceed = false;
                break;
            }
            InputEvent::Keyboard(KeyEvent::Right) => {
                let speed = (*state).turret.speed;

                if (*state).turret.x + speed < S_SIZE.0 as i32 {
                    (*state).turret.x += speed;
                }
            }
            InputEvent::Keyboard(KeyEvent::Left) => {
                let speed = (*state).turret.speed;

                if (*state).turret.x - speed >= 0 {
                    (*state).turret.x -= speed;
                }
            }
            InputEvent::Keyboard(KeyEvent::Up) => {
                let x = (*state).turret.x + 3;
                let y = (*state).turret.y - 2;

                (*state).bullets.push(Bullet {
                    speed: 20,
                    x: x,
                    y: y,
                    xf: x as f32,
                    yf: y as f32,
                });
            }
            _ => {}
        }
    }
}

fn update(
    state: &mut GameState,
    delta: Duration
) {
    for i in 0..(*state).bullets.len() {
        (*state).bullets[i].yf -=
            (*state).bullets[i].speed as f32 * delta.as_secs_f32();

        (*state).bullets[i].y = (*state).bullets[i].yf.round() as i32;
    }
}

fn render(
    state: &mut GameState,
    renderer: &mut Renderer
) ->
    Result<(), Box<dyn Error>>
{
    let mut render_queue =
        Vec::<(&Location, &Representation)>::new();

    let turret_location =
        Location {
            x: (*state).turret.x,
            y: (*state).turret.y,
        };

    render_queue.push((
        &turret_location,
        &(*state).views[(*state).turret.view]));

    let mut bullets_locations = Vec::<Location>::new();

    for i in 0..(*state).bullets.len() {
        bullets_locations.push(Location {
            x: (*state).bullets[i].x,
            y: (*state).bullets[i].y,
        });
    }

    for i in 0..bullets_locations.len() {
        render_queue.push((
            &bullets_locations[i],
            &(*state).views[(*state).bullet_view]));
    }

    renderer.render(&render_queue);

    Ok(())
}

fn load_view(
    filepath: &str, views: &mut Vec<Representation>
) ->
    Result<usize, Box<Error>>
{
    views.push(Representation::from_file(filepath)?);

    Ok(views.len() - 1)
}
