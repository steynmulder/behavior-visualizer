use sdl2::{render::Canvas, video::Window};

use crate::entity::conway_entity::ConwayEntity;
use std::collections::HashMap;

pub struct ConwayColony {
    entities: HashMap<(u32, u32), ConwayEntity>
}

impl ConwayColony {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { entities: map}
    }

    pub fn add(&mut self, coord: (u32, u32), entity: ConwayEntity) {
        self.entities.insert(coord, entity);
    }

    pub fn draw_entities(&mut self, canvas: &mut Canvas<Window>) {
        for entity in self.entities.values_mut().into_iter() {
            entity.draw(canvas);
        }
        let mut old_map = HashMap::new();
        for pair in <HashMap<(u32, u32), ConwayEntity> as Clone>::clone(&self.entities).into_iter() {
            old_map.insert(pair.0, pair.1);
        }
        for entity in self.entities.values_mut().into_iter() {
            let mut count: u8 = 0;
            for coord in entity.get_neighbors() {
                count += old_map.get_mut(coord).unwrap().get_alive() as u8;
            }
            entity.update(&count)
        }
    }

    pub fn get_entities(&mut self) -> &mut HashMap<(u32, u32), ConwayEntity> {
        &mut self.entities
    }
}