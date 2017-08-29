use sfml::graphics::*;
use sfml::system::Vector2f;
use super::entity::Entity;
use resources::Resources;

pub struct Bonfire<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Bonfire<'s> {
    pub fn new(res: &'s Resources) -> Bonfire<'s> {
        Self::with_position2f(res, 0., 0.)
    }

    pub fn with_position2f(res: &'s Resources, x: f32, y: f32) -> Bonfire<'s> {
        Self::with_position(res, &Vector2f::new(x, y))
    }

    pub fn with_position(res: &'s Resources, pos: &Vector2f) -> Bonfire<'s> {
        let mut b = Bonfire {
            sprite: Sprite::with_texture(&res.img.bonfire),
        };

        let size = res.img.bonfire.size();
        b.sprite.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);

        b.sprite.set_position(pos);

        b
    }
}

impl<'s> Entity<'s> for Bonfire<'s> {
    /*
    fn sprite(&self) -> &Sprite<'s> {
        &self.sprite
    }
    */

    fn global_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }

    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.sprite);
    }
}
