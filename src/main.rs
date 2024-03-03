extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
pub fn main() {
    let sdlContext = sdl2::init().unwrap();
    let video_subsystem = sdlContext.video().unwrap();
    let window = video_subsystem.window("Blank Title", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut Canvas = window.into_canvas().build().unwrap();
    Canvas.clear();
    Canvas.present();
    let mut drawing = false;
    let mut event_pump = sdlContext.event_pump().unwrap();
    Canvas.set_draw_color(Color::RGB(255, 255, 255));
    Canvas.clear();
    Canvas.present();
    Canvas.set_draw_color(Color::RGB(0, 0, 0));
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    drawing = true;
                }
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    drawing = false;
                }
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    if drawing {
                        Canvas.draw_point((x, y)).unwrap();
                    }
                }
                Event::Quit { .. } => break 'running,
                _ => {}
            }
            Canvas.present();
        }
    }
}