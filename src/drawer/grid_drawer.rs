extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn draw_grid(canvas: &mut Canvas<Window>, width: &u32, height: &u32) {
    // canvas.set_draw_color(Color::RGB(255, 255, 255));
    // canvas.clear();
    let cell_size = 16;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in 0..*width/cell_size {
        let _ = canvas.fill_rect(Rect::new((i * cell_size) as i32, 0, 3, *height));
    }
    for j in 0..*height/cell_size {
        let _ = canvas.fill_rect(Rect::new(0, (j * cell_size) as i32, *width, 3));
    }
}