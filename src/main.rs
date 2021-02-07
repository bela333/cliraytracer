
use std::{io::stdout, time::{Duration, Instant}, usize};

use crossterm::{ExecutableCommand, cursor::{self}, terminal::{self}};
use rand::Rng;
use shader::{eval, get_raytracer};
use terminal_size::{Height, Width, terminal_size};
use types::Parameters;

mod shader;
mod types;
mod utilities;
mod raytracer;

const PALETTE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const FONT_ASPECT_RATIO: f32 = 12.0/16.0;
//const FONT_ASPECT_RATIO: f32 = 9.0/19.0;
const FRAMERATE: u64 = 60;


fn draw(w: u16, h: u16, param: Parameters){
    let mut rng = rand::thread_rng();
    let range = 1f32 / (PALETTE.len()-1) as f32 / 2f32;
    let wf32 = w as f32;
    let hf32 = h as f32;
    let aspect_ratio = wf32 / (hf32 / FONT_ASPECT_RATIO);
    for y in 0..h {
        for x in 0..w {
            let x = x as f32 / wf32;
            let y = y as f32 / hf32;
            let val = 1f32-(eval(x, y, aspect_ratio, &param) + rng.gen_range(-range..range)); //Mediocre try at temporal dithering
            let val = if val < 0f32 {0f32}else{val};
            let val = if val > 1f32 {1f32}else{val};
            let index = (val * (PALETTE.len()-1) as f32 + 0.5).floor() as usize;
            let char = PALETTE.chars().nth(index).unwrap();
            print!("{}", char);
        }
        println!()
    }
}

fn main() {
    let target_frame_time = Duration::from_millis(1000/FRAMERATE);
    let (w, h) = if let Some((Width(w), Height(h))) = terminal_size(){
        (w, h)
    }else{
        (20, 15)
    };
    let h = h-1;
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let t = Instant::now();
    loop {
        let time = t.elapsed().as_secs_f32();
        let frame_time = Instant::now();
        let raytracer = get_raytracer(time);
        let param = Parameters{
            time,
            raytracer: &raytracer
        };
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        draw(w, h, param);
        let elapsed = frame_time.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep( target_frame_time - elapsed);
        }
    }
    
}
