extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, mouse::{MouseButton, MouseState}};
use std::time::Duration;
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
            colony.add((i * cell_size, j * cell_size), ConwayEntity::new(i * cell_size, j * cell_size, (i+j) % 2 == 0, cell_size, WIDTH, HEIGHT));
        }
    }

    // let entities: &mut HashMap<(u32, u32), ConwayEntity> = colony.get_entities();
    // entities.get_mut(&(320, 320)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(320, 336)).unwrap().set_alive(true, &mut canvas);   
    // entities.get_mut(&(320, 352)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(304, 352)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(288, 336)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(352, 368)).unwrap().set_alive(true, &mut canvas);   
    // entities.get_mut(&(368, 352)).unwrap().set_alive(true, &mut canvas);
    // entities.get_mut(&(368, 368)).unwrap().set_alive(true, &mut canvas);

    let mut draw_grid = true;
    let mut run_sim = true;
    let mut changed_cells : Vec<(u32, u32)> = vec![];
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;},
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {draw_grid = !draw_grid;},
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {run_sim = !run_sim;},
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let coord = ((x - (x % (cell_size as i32))) as u32, (y - (y % (cell_size as i32))) as u32);
                    colony.flip_entity(coord);
                    changed_cells.push(coord);

                }
                Event::MouseMotion { mousestate, x, y, xrel, yrel, .. } => {
                    let coord = ((x - (x % (cell_size as i32))) as u32, (y - (y % (cell_size as i32))) as u32);
                    if mousestate.left() && !changed_cells.contains(&coord) {
                        colony.flip_entity(coord);
                        changed_cells.push(coord);
                    }
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {changed_cells = vec![];}
                _ => {}
            }
        }
        colony.draw_entities(&mut canvas);
        if run_sim {
            colony.update_entities();
        }
        if draw_grid {
            grid_drawer::draw_grid(&mut canvas, &WIDTH, &HEIGHT, cell_size);
        }
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}