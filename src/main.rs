
use std::{f32::consts::PI, io::stdout, time::{Duration, Instant}, usize};

use crossterm::{ExecutableCommand, cursor::{self}, event::{EnableMouseCapture, Event, KeyCode, KeyModifiers, poll, read}, style::Colorize, terminal::{self}};
use rand::Rng;
use ray_resolvers::bvh::generate_bvh_from_file;
use shader::{eval, get_params};
use terminal_size::{Height, Width, terminal_size};
use types::Parameters;
use utilities::{Matrix3, Vector3};

mod shader;
mod types;
mod utilities;
mod raytracer;
mod ray_resolvers;
mod error;

const PALETTE: &str = "@&%QWNM0gB$#DR8mHXKAbUGOpV4d9h6PqkwSE2]ayjxY5Zeonu[l1t3If}C{Fi|(7J)vTLsz?:/*cr!+<>;\"=~^,_:'-.` ";
const FONT_ASPECT_RATIO: f32 = 8.0/16.0;   //Aspect ratio of font. width/height. 0.5 seems to work well for anything other than windows command prompt
const FRAMERATE: u64 = 60;

const HALFTONE_SIZE: usize = 4;
const HALFTONE: [[f32; HALFTONE_SIZE]; HALFTONE_SIZE] = [
    [0.0/15.0, 8.0/15.0, 2.0/15.0, 10.0/15.0],
    [12.0/15.0, 4.0/15.0, 14.0/15.0, 6.0/15.0],
    [3.0/15.0, 11.0/15.0, 1.0/15.0, 9.0/15.0],
    [15.0/15.0, 7.0/15.0, 13.0/15.0, 5.0/15.0],
];

fn draw(w: u16, h: u16, param: &Parameters){
    let range = 1f32 / (PALETTE.len()-1) as f32 / 2f32;
    let wf32 = w as f32;
    let hf32 = h as f32;
    let aspect_ratio = wf32 / (hf32 / FONT_ASPECT_RATIO);
    for y in 0..h {
        let y_f32 = y as f32 / hf32;
        for x in 0..w {
            let x_f32 = x as f32 / wf32;
            let dither_value = HALFTONE[x as usize%HALFTONE_SIZE][y as usize%HALFTONE_SIZE]-0.5;
            let val = (1f32-eval(x_f32, y_f32, aspect_ratio, param)) + dither_value*range;
            let val = if val < 0f32 {0f32}else{val};
            let val = if val > 1f32 {1f32}else{val};
            let index = (val * (PALETTE.len()-1) as f32 + 0.5).floor() as usize;
            let character = PALETTE.chars().nth(index).unwrap();
            print!("{}", character);
        }
        println!();
    }
}

fn main() {
    let target_frame_time = Duration::from_millis(1000/FRAMERATE);
    let  (mut w, mut h) = if let Some((Width(w), Height(h))) = terminal_size(){
        (w, h-1)
    }else{
        (20, 15)
    };
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(EnableMouseCapture).unwrap();
    let t = Instant::now();
    let mut camera_pos = Vector3::zero();
    let mut current_speed = Vector3::zero();
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mesh = generate_bvh_from_file("teapot.obj").unwrap();
    let mut crouch = false;
    loop {
        let time = t.elapsed().as_secs_f32();
        let direction = Matrix3::identity();
        let direction = direction.rotateX(pitch);
        let direction = direction.rotateY(yaw);
        let param = get_params(time, camera_pos, direction, &mesh);
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let frame_time = Instant::now();
        draw(w, h, &param);
        let elapsed = frame_time.elapsed();
        const SPEED: f32 = 1.0;
        let mut target_speed = Vector3::zero();
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    match key.code{
                        KeyCode::Char('w') => {
                            let mut d = direction.k;
                            d.y = 0.0;
                            d = d.normalized().multiply(SPEED);
                            target_speed = d;
                        },
                        KeyCode::Char('s') => {
                            let mut d = direction.k;
                            d.y = 0.0;
                            d = d.normalized().multiply(SPEED);
                            target_speed = d.negate();
                        },
                        KeyCode::Char('a') => {
                            let mut d = direction.i;
                            d.y = 0.0;
                            d = d.normalized().multiply(SPEED);
                            target_speed = d.negate();
                        },
                        KeyCode::Char('d') => {
                            let mut d = direction.i;
                            d.y = 0.0;
                            d = d.normalized().multiply(SPEED);
                            target_speed = d;
                        },
                        KeyCode::Char('c') => {
                            camera_pos.y = match crouch {
                                true => -0.5,
                                false => 0.0,
                            };
                            crouch = !crouch;
                        }
                        _ => ()
                    }
                }
                Event::Mouse(e)=>{
                    yaw = (e.column as f32/w as f32)*2.0*PI-PI;
                    pitch = (e.row as f32/h as f32)*PI-PI/2.0;
                }
                Event::Resize(_w, _h)  => {
                    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                    w = _w;
                    h = _h-1;
                }
                _ => (),
            }
        }
        const CURVE: f32 = 0.5;
        current_speed = target_speed.multiply(CURVE).add(current_speed.multiply(1.0-CURVE));
        camera_pos = camera_pos.add(current_speed);
        if elapsed < target_frame_time {
            std::thread::sleep( target_frame_time - elapsed);
        }
    }
    
}
