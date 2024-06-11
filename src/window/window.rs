extern crate sdl2;

use sdl2::event::Event;
use std::{collections::HashMap, time::Duration};
use crate::{colony::conway_colony::ConwayColony, drawer::grid_drawer, entity::conway_entity::ConwayEntity};

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


    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut colony = ConwayColony::new();
    let cell_size = 16;

    for i in 0..(WIDTH/cell_size) {
        for j in 0..(HEIGHT/cell_size) {
            colony.add((i * cell_size, j * cell_size), ConwayEntity::new(i * cell_size, j * cell_size, false, cell_size, WIDTH, HEIGHT));
        }
    }

    let entities: &mut HashMap<(u32, u32), ConwayEntity> = colony.get_entities();
    entities.get_mut(&(320, 320)).unwrap().set_alive(true, &mut canvas);
    entities.get_mut(&(336, 320)).unwrap().set_alive(true, &mut canvas);   
    // entities.get_mut(&(336, 336)).unwrap().set_alive(true, &mut canvas);
    entities.get_mut(&(320, 336)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(352, 352)).unwrap().set_alive(true, &mut canvas);
    entities.get_mut(&(352, 368)).unwrap().set_alive(true, &mut canvas);   
    entities.get_mut(&(368, 352)).unwrap().set_alive(true, &mut canvas);
    entities.get_mut(&(368, 368)).unwrap().set_alive(true, &mut canvas);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;},
                _ => {}
            }
        }

        grid_drawer::draw_grid(&mut canvas, &WIDTH, &HEIGHT);
        colony.draw_entities(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
    }
}