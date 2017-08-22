use sfml::graphics::RenderTarget;
use super::entity::*;

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
            target.draw(i.sprite());
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
