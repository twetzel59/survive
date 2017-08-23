use sfml::graphics::RenderTarget;
use sfml::system::Vector2i;
use super::entity::*;
use mouse_handler::MouseClickHander;

pub struct EntityManager<'s> {
    entities: Vec<Box<Entity<'s> + 's>>,
}

impl<'s> EntityManager<'s> {
    pub fn new() -> EntityManager<'s> {
        //let mut entities: Vec<Box<Entity<'s>>> = Vec::new();
        //entities.push(Box::new(DeciduousTree::new(res)));

        EntityManager {
            entities: Vec::new(),
        }
    }

    pub fn draw_all<T: RenderTarget>(&self, target: &mut T) {
        for i in &self.entities {
            i.draw(target);
        }
    }

    /*
    pub fn add<T: Entity<'s> + 'a>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
    }
    */

    /*
    pub fn add(&mut self, entity: Box<Entity<'s> + 's>) {
        self.entities.push(entity);
    }
    */

    pub fn add<T: Entity<'s> + 's>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
    }
}

impl<'s> MouseClickHander for EntityManager<'s> {
    fn mouse_click(&mut self, target: &RenderTarget, mx: i32, my: i32) {
        let coords = target.map_pixel_to_coords_current_view(&Vector2i::new(mx, my));

        println!("EntityManager click: {:?}", coords);

        for i in &mut self.entities {
            if i.sprite().global_bounds().contains(coords) {
                i.on_click();
            }
        }
    }
}
