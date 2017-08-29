use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};
use super::entity::Entity;
use resources::Resources;

pub struct Bonfire<'s> {
    obj: Sprite<'s>,
    anim: Sprite<'s>,
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
            obj: Sprite::with_texture(&res.img.bonfire),
            anim: Sprite::with_texture(&res.img.fire_atlas.tex),
        };

        let size = res.img.bonfire.size();
        b.obj.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        b.obj.set_position(pos);

        let size = res.img.fire_atlas.tex.size() / Vector2u::new(1, res.img.fire_atlas.n_frames);

        b.anim.set_texture_rect(&IntRect::new(0, 0, size.x as i32, size.y as i32));
        b.anim.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        b.anim.set_position(pos);

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
        self.obj.global_bounds()
    }

    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.obj);
        target.draw(&self.anim);
    }

    fn update(&mut self, _delta: f32) {

    }
}
