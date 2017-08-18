use sfml::graphics::{RenderTarget, Sprite};
use super::entity::*;
use resources::Resources;

pub struct EntityManager<'a, 's> {
    entities: Vec<Box<Entity<'s> + 'a>>,
}

impl<'a, 's> EntityManager<'a, 's> {
    pub fn new() -> EntityManager<'a, 's> {
        //let mut entities: Vec<Box<Entity<'s>>> = Vec::new();
        //entities.push(Box::new(DeciduousTree::new(res)));
        
        EntityManager {
            entities: Vec::new(),
        }
    }
    
    pub fn draw_all<T: RenderTarget>(&self, target: &mut T) {
        for i in &self.entities {
            target.draw(i.sprite());
        }
    }
    
    pub fn add<T: Entity<'s> + 'a>(&mut self, entity: T) {
        self.entities.push(Box::new(entity));
    }
}
