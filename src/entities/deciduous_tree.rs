use rand::{self, Rng};
use sfml::graphics::*;
use sfml::system::Vector2f;
use super::entity::Entity;
use registry::item::{Item, Stack};
use resources::Resources;

const DROP_MEDIAN: u8 = 10;
const DROP_VARIATION: u8 = 3;

pub struct DeciduousTree<'s> {
    harvested: bool,
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
            harvested: false,
            sprite: Sprite::with_texture(&res.img.deciduous),
        };

        //d.sprite.set_color(&Color::rgba(255, 255, 255, 100));

        let size = res.img.deciduous.size();
        d.sprite.set_origin2f(size.x as f32 / 2., size.y as f32 / 2.);

        d.sprite.set_position(pos);

        d
    }

    /*
    pub fn harveted(&self) -> {
        self.harvested
    }
    */

    fn harvest(&mut self) -> bool {
        if self.harvested {
            return false;
        }

        self.harvested = true;
        self.sprite.set_color(&Color::rgba(200, 200, 200, 50));

        true
    }
}

impl<'s> Entity<'s> for DeciduousTree<'s> {
    /*
    fn sprite(&self) -> &Sprite<'s> {
        &self.sprite
    }
    */

    /*
    fn sprite_mut(&mut self) -> &mut Sprite<'s> {
        &mut self.sprite
    }
    */

    fn global_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }

    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.sprite);
    }

    fn care_about_click(&self) -> bool {
        true
    }

    fn on_click(&mut self) -> Stack {
        if self.harvest() {
            Stack { item: Item::Wood,
                          count: rand::thread_rng().gen_range(
                                       DROP_MEDIAN - DROP_VARIATION, DROP_MEDIAN + DROP_VARIATION) }
        } else {
            Stack { item: Item::Wood, count: 0 }
        }
    }
}
