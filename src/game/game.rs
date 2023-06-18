use crate::particle::Particle;
use ndarray::{arr1, Array1};
use rand::Error;
use rand::{rngs::ThreadRng, Rng};
use rstar::primitives::Rectangle;
use rstar::RTree;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use std::time::Duration;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

const radius: f64 = 50.0;
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

const INITIAL_X: f64 = 40.0;
const INITIAL_Y: f64 = 25.0;

pub struct GameObjects {
    particles: Vec<Particle>,
    atractors: Vec<Array1<f64>>,
}
pub struct GameState {
    context: Sdl,
    video_subsystem: VideoSubsystem,
    game_objects: GameObjects,
    number_of_particles: i32,
}

fn point_on_circle(k: f64, n: f64) -> (f64, f64) {
    let x = 49.0 + radius * (2.0 * k * PI / n).cos();
    let y = 49.0 + radius * (2.0 * k * PI / n).sin();

    return (x, y);
}

pub fn game_init(number_of_particles: usize, rng: &mut ThreadRng) -> Result<GameState, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut game_state = GameState {
        context: sdl_context,
        video_subsystem: video_subsystem,
        game_objects: GameObjects {
            particles: Vec::with_capacity(number_of_particles),
            atractors: Vec::new(),
        },
        number_of_particles: number_of_particles as i32,
    };

    for index in 0..number_of_particles {
        game_state.game_objects.particles.push(Particle::new(
            INITIAL_X + (index as f64) / 10000.0,
            INITIAL_Y + (index as f64) / 10000.0,
            0.025,
            0.0,
        ));
    }

    //game_state.game_objects.atractors.push(arr1(&[49.0, 49.0]));
    game_state.game_objects.atractors.push(arr1(&[79.0, 49.0]));
    game_state.game_objects.atractors.push(arr1(&[19.0, 49.0]));

    return Ok(game_state);
}

fn do_game_tick(game_state: &mut GameState, rng: &mut ThreadRng) -> Result<i32, String> {
    for (_, particle) in game_state.game_objects.particles.iter_mut().enumerate() {
        for atractor in game_state.game_objects.atractors.iter() {
            particle.apply_physics(1.0, atractor);
        }
    }

    return Ok(0);
}

fn render(canvas: &mut WindowCanvas, game_state: &mut GameState) -> Result<i32, String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    for (i, particle) in game_state.game_objects.particles.iter().enumerate() {
        canvas.set_draw_color(Color::RGB(
            (255.0 * (i as f32 / game_state.number_of_particles as f32)) as u8,
            0,
            0,
        ));

        let x = (WIDTH as f32 / 101 as f32 * particle.x as f32) as i32;
        let width = (WIDTH as f32 / 50 as f32) as u32;
        let y = (HEIGHT as f32 / 101 as f32 * particle.y as f32) as i32;
        let height = (HEIGHT as f32 / 50 as f32) as u32;

        canvas.fill_rect(Rect::new(x, y, width, height))?;
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    for atractor in game_state.game_objects.atractors.iter() {
        let x = (WIDTH as f32 / 101 as f32 * atractor[0] as f32) as i32;
        let width = (WIDTH as f32 / 101 as f32) as u32;
        let y = (HEIGHT as f32 / 101 as f32 * atractor[1] as f32) as i32;
        let height = (HEIGHT as f32 / 101 as f32) as u32;

        canvas.fill_rect(Rect::new(x, y, width + 1, height + 1))?;
    }
    /*
    {
        let x = (WIDTH as f32 / 101 as f32 * 70 as f32) as i32;
        let width = (WIDTH as f32 / 101 as f32) as u32;
        let y = (HEIGHT as f32 / 101 as f32 * 49 as f32) as i32;
        let height = (HEIGHT as f32 / 101 as f32) as u32;

        canvas.fill_rect(Rect::new(x, y, width + 1, height + 1))?;
    }
    */

    canvas.present();

    return Ok(0);
}

pub fn run_game(game_state: GameState, rng: &mut ThreadRng) -> Result<i32, String> {
    let window = game_state
        .video_subsystem
        .window("rust particles", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut game_state = game_state;

    let mut event_pump = game_state.context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let _game_tick = do_game_tick(&mut game_state, rng).unwrap();

        let _render_status = render(&mut canvas, &mut game_state).unwrap();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }

    return Ok(0);
}
