mod renderer;
mod gameloop;

use std::error::Error;
use std::time::Duration;
use std::panic;
use std::thread;

use crossterm::input::InputEvent;
use crossterm::input::KeyEvent;
use rand::Rng;

use renderer::types::Location;
use renderer::types::Renderer;
use renderer::types::Representation;

// screen size
const S_SIZE: (u16, u16) = (50, 30);
const FPS: u8 = 30;

struct Enemy {
    pub xf: f32,
    pub yf: f32,
    pub target_x: i32,
    pub target_y: i32,
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub speedup: u8
}

struct Bullet {
    pub xf: f32,
    pub yf: f32,
    pub x: i32,
    pub y: i32,
    pub speed: i32,
}

struct Turret {
    pub x: i32,
    pub y: i32,
    pub view: usize,
    pub speed: i32,
}

struct GameState {
    views: Vec<Representation>,
    turret: Turret,
    bullet_view: usize,
    bullets: Vec<Bullet>,
    enemy_view: usize,
    enemies: Vec<Enemy>,
    ammo: u8,
    score: u16
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logging::log_to_file(
        "output.log", log::LevelFilter::Info)?;
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        match info.location() {
            Some(location) => {
                log::error!(
                    "thread '{}' panicked at '{}': {}:{}",
                    thread,
                    msg,
                    location.file(),
                    location.line(),
                );
            }
            None => {
                log::error!(
                    "thread '{}' panicked at '{}'",
                    thread,
                    msg,
                )
            }
        }
    }));

    let renderer =
        renderer::init(S_SIZE.0 + 20, S_SIZE.1)?;

    let mut views = Vec::<Representation>::new();

    let turret_view =
        load_view(
            "./res/objects/turret.yaml", &mut views)?;
    let bullet_view =
        load_view(
            "./res/objects/bullet.yaml", &mut views)?;
    let enemy_view =
        load_view(
            "./res/objects/enemy.yaml", &mut views)?;

    let state = GameState {
        views: views,
        turret: Turret {
            speed: 4,
            x: S_SIZE.0 as i32 / 2 - 5,
            y: S_SIZE.1 as i32 - 2,
            view: turret_view
        },
        bullet_view: bullet_view,
        bullets: Vec::<Bullet>::new(),
        enemy_view: enemy_view,
        enemies: create_enemies(),
        ammo: 3,
        score: 0
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

                let turret_width = (*state).views[(*state).turret.view].data()[0].len() as i32;

                if (*state).turret.x + speed + turret_width < S_SIZE.0 as i32 {
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
                if (*state).ammo > 0 {
                    let x = (*state).turret.x + 3;
                    let y = (*state).turret.y - 0;

                    (*state).ammo -= 1;

                    (*state).bullets.push(Bullet {
                        speed: -20,
                        x: x,
                        y: y,
                        xf: x as f32,
                        yf: y as f32,
                    });
                }
            }
            _ => {}
        }
    }
}

fn update(
    state: &mut GameState,
    delta: Duration
) {
    let mut enemies_on_removal = Vec::<usize>::new();
    let mut new_enemies = Vec::<Enemy>::new();

    for i in 0..(*state).enemies.len() {
        let enemy_ptr = &mut (*state).enemies[i];

        (*enemy_ptr).target_x = (*state).turret.x;
        (*enemy_ptr).target_y = (*state).turret.y;

        (*enemy_ptr).yf +=
            (*enemy_ptr).speed as f32 * delta.as_secs_f32();

        (*enemy_ptr).y = (*enemy_ptr).yf.round() as i32;

        if
            (*enemy_ptr).y < 0 ||
            (*enemy_ptr).y > S_SIZE.1 as i32
        {
            enemies_on_removal.push(i);
        }
    }



    let mut bullets_on_removal = Vec::<usize>::new();

    for i in 0..(*state).bullets.len() {
        (*state).bullets[i].yf +=
            (*state).bullets[i].speed as f32 * delta.as_secs_f32();

        (*state).bullets[i].y = (*state).bullets[i].yf.round() as i32;

        if
            (*state).bullets[i].y < 0 ||
            (*state).bullets[i].y > S_SIZE.1 as i32
        {
            bullets_on_removal.push(i);
        }
    }



    let bullet_width = (*state).views[(*state).bullet_view].data()[0].len() as f32;
    let bullet_height = (*state).views[(*state).bullet_view].data().len() as f32;

    let enemy_width = (*state).views[(*state).enemy_view].data()[0].len() as f32;
    let enemy_height = (*state).views[(*state).enemy_view].data().len() as f32;

    for i in 0..(*state).bullets.len() {
        let bullet_ptr = &mut (*state).bullets[i];
        let bullet_delta = (*bullet_ptr).speed as f32 * delta.as_secs_f32();

        for j in 0..(*state).enemies.len() {
            let enemy_ptr = &mut (*state).enemies[j];
            let enemy_delta = (*enemy_ptr).speed as f32 * delta.as_secs_f32();

            // log::info!("{:?}", "-----");
            // log::info!("{:?}, (*enemy_ptr).xf", (*enemy_ptr).xf);
            // log::info!("{:?}, (*bullet_ptr).xf", (*bullet_ptr).xf);
            // log::info!("{:?}, bullet_width", bullet_width);
            // log::info!("{:?}, enemy_width", enemy_width);
            // log::info!("{:?}, (*enemy_ptr).yf", (*enemy_ptr).yf);
            // log::info!("{:?}, enemy_delta", enemy_delta);
            // log::info!("{:?}, (*bullet_ptr).yf", (*bullet_ptr).yf);
            // log::info!("{:?}, bullet_height", bullet_height);
            // log::info!("{:?}, bullet_delta", bullet_delta);
            // log::info!("{:?}, enemy_height", enemy_height);

            if
                (*enemy_ptr).xf <= (*bullet_ptr).xf + bullet_width &&
                (*bullet_ptr).xf <= (*enemy_ptr).xf + enemy_width &&

                (*enemy_ptr).yf - enemy_delta <= (*bullet_ptr).yf + bullet_height - bullet_delta &&
                (*bullet_ptr).yf <= (*enemy_ptr).yf + enemy_height
            {
                bullets_on_removal.push(i);
                enemies_on_removal.push(j);
                (*enemy_ptr).speedup = 1;
                (*state).score += 1;
            }
        }
    }



    bullets_on_removal.sort();
    bullets_on_removal.reverse();
    bullets_on_removal.dedup();
    for i in bullets_on_removal {
        (*state).bullets.remove(i);
        (*state).ammo += 1;
    }

    enemies_on_removal.sort();
    enemies_on_removal.reverse();
    enemies_on_removal.dedup();
    for i in enemies_on_removal.iter().cloned() {
        let removed_enemy = (*state).enemies.remove(i);
        new_enemies.push(
            create_enemy(
                removed_enemy.speed + removed_enemy.speedup as i32, 0
            )
        );
    }

    while let Some(new_enemy) = new_enemies.pop() {
        (*state).enemies.push(new_enemy);
    }
}

fn render(
    state: &mut GameState,
    renderer: &mut Renderer
) ->
    Result<(), Box<dyn Error>>
{
    let mut wall_view_data = Vec::<Vec<char>>::new();

    for _i in 0..S_SIZE.1 {
        wall_view_data.push(vec!['|', '|']);
    }

    let wall_view =
        Representation::new(' ', wall_view_data);

    let bullets_lable_view =
        Representation::new(' ',
            vec![vec!['A', 'M', 'M', 'O', ':']]);
    let bullets_lable_location =
        Location {
            x: (S_SIZE.0 + 7) as i32,
            y: (S_SIZE.1 - 3) as i32
        };

    let score_lable_view =
        Representation::new(' ',
            vec![vec!['S', 'C', 'O', 'R', 'E', ':']]);
    let score_lable_location =
        Location {
            x: (S_SIZE.0 + 9) as i32,
            y: 2 as i32
        };

    let score_view =
        Representation::new(' ',
            vec![(*state).score.to_string().chars().collect()]);
    let score_location =
        Location {
            x: (S_SIZE.0 + 9) as i32,
            y: 3 as i32
        };

    let mut render_queue =
        Vec::<(&Location, &Representation)>::new();

    let wall_location =
        Location {
            x: (S_SIZE.0 + 1) as i32,
            y: 0,
        };

    let turret_location =
        Location {
            x: (*state).turret.x,
            y: (*state).turret.y,
        };

    render_queue.push((
        &turret_location,
        &(*state).views[(*state).turret.view]));

    render_queue.push((
        &wall_location,
        &wall_view));

    render_queue.push((
        &bullets_lable_location,
        &bullets_lable_view));

    render_queue.push((
        &score_lable_location,
        &score_lable_view));

    render_queue.push((
        &score_location,
        &score_view));



    let mut bullets_locations = Vec::<Location>::new();

    for i in 0..(*state).bullets.len() {
        bullets_locations.push(Location {
            x: (*state).bullets[i].x,
            y: (*state).bullets[i].y,
        });
    }

    for i in 0..(*state).ammo {
        let x = S_SIZE.0 + 13;
        let y = S_SIZE.1 - 3;

        bullets_locations.push(Location {
            x: x as i32 + i as i32,
            y: y as i32,
        });
    }

    for i in 0..bullets_locations.len() {
        render_queue.push((
            &bullets_locations[i],
            &(*state).views[(*state).bullet_view]));
    }



    let mut enemies_locations = Vec::<Location>::new();

    for i in 0..(*state).enemies.len() {
        enemies_locations.push(Location {
            x: (*state).enemies[i].x,
            y: (*state).enemies[i].y,
        });
    }

    for i in 0..enemies_locations.len() {
        render_queue.push((
            &enemies_locations[i],
            &(*state).views[(*state).enemy_view]));
    }



    renderer.render(&render_queue)?;

    Ok(())
}

fn load_view(
    filepath: &str, views: &mut Vec<Representation>
) ->
    Result<usize, Box<dyn Error>>
{
    views.push(Representation::from_file(filepath)?);

    Ok(views.len() - 1)
}

fn create_enemies() -> Vec<Enemy> {
    let mut enemies = Vec::<Enemy>::new();

    for i in 0..3 {
        enemies.push(create_enemy(10, 5 * i));
    }

    enemies
}

fn create_enemy(spd: i32, y: i32) -> Enemy {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0, (S_SIZE.0 as i32) - 9);

    Enemy {
        xf: x as f32,
        yf: y as f32,
        x: x,
        y: y,
        target_x: x,
        target_y: y,
        speed: spd,
        speedup: 0
    }
}
