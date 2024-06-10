extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub fn draw_grid(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(100, 100, 0));
    canvas.clear();
}