use std::io;

use beryllium::*;
use beryllium::init::InitFlags;
use beryllium::events::Event;
use beryllium::video::CreateWinArgs;
use beryllium::video::RendererFlags;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 600;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {

    let mut counter = 0;

    let sdl = Sdl::init(InitFlags::EVERYTHING);

    let window = sdl
        .create_renderer_window(CreateWinArgs { title: "COLLATZ", width: WIDTH, height: HEIGHT, ..Default::default()},
                                                RendererFlags::ACCELERATED)
        .unwrap();

    let mut points = Vec::new();
      'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        counter += 1;
        points.push(Point {x: counter, y: collatz(counter)});
        println!("Info:: added new point: {:?}", points[points.len()-1]);
    
        window.set_draw_color(255, 255, 255, 254).unwrap();
        window.clear().unwrap();

        window.set_draw_color(0, 0, 0, 255).unwrap();
        for point in &points {
        window.draw_points(&[[point.x, point.y]]).unwrap();
        }
        window.present();
    }

}

 
fn collatz(n: i32) -> i32{
    let mut g:u128 = n.try_into().unwrap();
    let mut i = 0;
    loop {
        if g == 1 {
            return i;
        } else if g % 2 == 0 {
            g = g/2;
            i+=1;
        } else {
            g = 3*g + 1;
            i+=1;
        }
    }

} 
