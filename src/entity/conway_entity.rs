
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Clone)]
pub struct ConwayEntity {
    x: u32,
    y: u32,
    alive: bool,
    neighbors: Vec<(u32, u32)>, 
    cell_size: u32
}

impl ConwayEntity {
    pub fn new(x: u32, y: u32, alive: bool, cell_size: u32, width: u32, height: u32) -> Self {
        let mut neighbors: Vec<(u32, u32)> = vec![];
        if x > cell_size {
            neighbors.push((x - cell_size, y));
            if y > cell_size {
                neighbors.push((x - cell_size, y - cell_size));
            }

            if y + cell_size < height {
                neighbors.push((x - cell_size, y + cell_size));
            }
        }

        if x + cell_size < width {
            neighbors.push((x + cell_size, y));

            if y > cell_size {
                neighbors.push((x + cell_size, y - cell_size));
            }

            if y + cell_size < height {
                neighbors.push((x + cell_size, y + cell_size));
            }
        }

        if y > cell_size {
            neighbors.push((x, y - cell_size));
        }

        if y + cell_size < height {
            neighbors.push((x, y + cell_size));
        }

        Self {x: x, y: y, alive: alive, neighbors: neighbors, cell_size: cell_size}
        
    }
    pub fn update(&mut self, living_neighbors: &u8) {

        match *living_neighbors {
            x if x < 2 => {self.alive = false;},
            x if self.alive && x < 4 || (x == 3 && !self.alive) => {self.alive = true;},
            _ => {self.alive = false;}
        }

    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if self.alive {
            canvas.set_draw_color(Color::RGB(200, 200, 0));   
        } else {
            canvas.set_draw_color(Color::RGB(0, 200, 200));  
        }
        let _ = canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, self.cell_size, self.cell_size));
    
    }

    pub fn get_alive(&self) -> bool {
        self.alive
    }

    pub fn get_neighbors(&self) -> &Vec<(u32, u32)> {
        &self.neighbors
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn set_alive(&mut self, alive: bool, canvas: &mut Canvas<Window>) {
        self.alive = alive;
        if self.alive {
            canvas.set_draw_color(Color::RGB(200, 200, 0));   
        } else {
            canvas.set_draw_color(Color::RGB(0, 200, 200));  
        }
        let _ = canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, self.cell_size, self.cell_size));
    }

    pub fn flip_alive(&mut self) {
        self.alive = !self.alive;
    }
}