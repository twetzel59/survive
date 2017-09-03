use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};
use super::entity::{Entity, EntityKind};
use super::texture_animator::TextureAnimator;
use resources::Resources;

pub struct Bonfire<'s> {
    wood: Sprite<'s>,
    flame: Sprite<'s>,
    animator: TextureAnimator,
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
            wood: Sprite::with_texture(&res.img.bonfire),
            flame: Sprite::with_texture(&res.img.fire_atlas.tex),
            animator: TextureAnimator::new(&res.img.fire_atlas),
        };

        let size = res.img.bonfire.size();
        b.wood.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        b.wood.set_position(pos);

        let size = res.img.fire_atlas.tex.size() / Vector2u::new(1, res.img.fire_atlas.n_frames);

        b.flame.set_texture_rect(&IntRect::new(0, 0, size.x as i32, size.y as i32));
        b.flame.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);
        b.flame.set_position(pos);

        b
    }
}

impl<'s> Entity<'s> for Bonfire<'s> {
    fn kind(&self) -> EntityKind {
        EntityKind::Bonfire
    }

    /*
    fn sprite(&self) -> &Sprite<'s> {
        &self.sprite
    }
    */

    fn daylight_affects(&self) -> bool {
        false
    }

    fn global_bounds(&self) -> FloatRect {
        self.wood.global_bounds()
    }

    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.wood);
        target.draw(&self.flame);
    }

    fn update(&mut self, delta: f32) {
        self.animator.update(delta);

        self.flame.set_texture_rect(&self.animator.texture_rect());
    }
}
