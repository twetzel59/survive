use sfml::graphics::*;
use sfml::system::Vector2f;
use super::entity::Entity;
use resources::Resources;

pub struct DeciduousTree<'s> {
    sprite: Sprite<'s>,
}

impl<'s> DeciduousTree<'s> {
    pub fn new(res: &'s Resources) -> DeciduousTree<'s> {
        Self::with_position2f(res, 0., 0.)
    }
    
    pub fn with_position2f(res: &'s Resources, x: f32, y: f32) -> DeciduousTree<'s> {
        Self::with_position(res, &Vector2f::new(x, y))
    }
    
    pub fn with_position(res: &'s Resources, pos: &Vector2f) -> DeciduousTree<'s> {
        let mut d = DeciduousTree {
            sprite: Sprite::with_texture(&res.img.deciduous),
        };
        
        d.sprite.set_color(&Color::rgba(255, 255, 255, 100));
        
        let size = res.img.deciduous.size();
        d.sprite.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        
        d.sprite.set_position(pos);
        
        d
    }
}

impl<'s> Drawable for DeciduousTree<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
                                   &'se self, 
                                   target: &mut RenderTarget, 
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(&self.sprite, states);
    }
}

impl<'s> Entity<'s> for DeciduousTree<'s> {
    fn sprite(&self) -> &Sprite<'s> {
        &self.sprite
    }
    
    fn sprite_mut(&mut self) -> &mut Sprite<'s> {
        &mut self.sprite
    }
}
