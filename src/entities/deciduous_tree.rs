use sfml::graphics::*;
use super::entity::Entity;
use resources::Resources;

pub struct DeciduousTree<'s> {
    sprite: Sprite<'s>,
}

impl<'s> DeciduousTree<'s> {
    pub fn new(res: &'s Resources) -> DeciduousTree<'s> {
        DeciduousTree {
            sprite: Sprite::with_texture(&res.img.deciduous),
        }
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
