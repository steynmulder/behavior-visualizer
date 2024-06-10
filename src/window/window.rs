extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;
use crate::drawer::grid_drawer;

pub fn create_window() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 960;

    // barebones from https://docs.rs/sdl2/latest/sdl2/
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("visualizer", WIDTH, HEIGHT)
        // .position_centered()
        .position(0, 40)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;},
                _ => {}
            }
        }
        grid_drawer::draw_grid(&mut canvas, &WIDTH, &HEIGHT);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}