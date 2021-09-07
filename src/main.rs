
use std::{f32::consts::PI, io::stdout, time::{Duration, Instant}, usize};

use crossterm::{ExecutableCommand, cursor::{self}, event::{EnableMouseCapture, Event, KeyCode, poll, read}, style::Colorize, terminal::{self}};
use rand::Rng;
use shader::{eval, get_params};
use terminal_size::{Height, Width, terminal_size};
use types::Parameters;
use utilities::{Matrix3, Vector3};

mod shader;
mod types;
mod utilities;
mod raytracer;

const PALETTE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const FONT_ASPECT_RATIO: f32 = 12.0/16.0;   //Aspect ratio of font. width/height. 0.5 seems to work well for anything other than windows command prompt
const FRAMERATE: u64 = 60;


fn draw(w: u16, h: u16, param: &Parameters){
    let mut rng = rand::thread_rng();   //RNG used for temporal dithering
    let range = 1f32 / (PALETTE.len()-1) as f32 / 2f32;
    let wf32 = w as f32;
    let hf32 = h as f32;
    let aspect_ratio = wf32 / (hf32 / FONT_ASPECT_RATIO);
    for y in 0..h {
        let y = y as f32 / hf32;
        for x in 0..w {
            let x = x as f32 / wf32;
            let val = 1f32-(eval(x, y, aspect_ratio, param) + rng.gen_range(-range..range)); //Mediocre try at temporal dithering
            //let val = 1f32-(eval(x, y, aspect_ratio, param));
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
    loop {
        let time = t.elapsed().as_secs_f32();
        let direction = Matrix3::identity();
        let direction = direction.rotateX(pitch);
        let direction = direction.rotateY(yaw);
        let param = get_params(time, camera_pos, direction);
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let frame_time = Instant::now();
        draw(w, h, &param);
        let elapsed = frame_time.elapsed();
        const SPEED: f32 = 2.0;
        let mut target_speed = Vector3::zero();
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(key) => {
                    match key.code{
                        KeyCode::Char('w') => {
                            let mut d = direction.k;
                            d.y = 0.0;
                            d = d.multiply(SPEED);
                            target_speed = d;
                        },
                        KeyCode::Char('s') => {
                            let mut d = direction.k;
                            d.y = 0.0;
                            d = d.multiply(SPEED);
                            target_speed = d.negate();
                        },
                        KeyCode::Char('a') => {
                            let mut d = direction.i;
                            d.y = 0.0;
                            d = d.multiply(SPEED);
                            target_speed = d.negate();
                        },
                        KeyCode::Char('d') => {
                            let mut d = direction.i;
                            d.y = 0.0;
                            d = d.multiply(SPEED);
                            target_speed = d;
                        },
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
        const CURVE: f32 = 0.1;
        current_speed = target_speed.multiply(CURVE).add(current_speed.multiply(1.0-CURVE));
        camera_pos = camera_pos.add(current_speed);
        if elapsed < target_frame_time {
            std::thread::sleep( target_frame_time - elapsed);
        }
    }
    
}
