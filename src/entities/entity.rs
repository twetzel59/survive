use sfml::graphics::{FloatRect, RenderTarget};
use registry::item::{Item, Stack};

#[derive(Copy, Clone)]
pub enum EntityKind {
    DeciduousTree,
    Bonfire,
}

pub trait Entity<'s> {
    fn kind(&self) -> EntityKind;
    fn draw(&self, target: &mut RenderTarget);
    //fn sprite(&self) -> &Sprite<'s>;
    //fn sprite_mut(&mut self) -> &mut Sprite<'s>;
    fn global_bounds(&self) -> FloatRect;

    fn daylight_affects(&self) -> bool {
        true
    }

    fn care_about_click(&self) -> bool {
        false
    }

    fn on_click(&mut self) -> Stack {
        Stack { item: Item::Wood, count: 0 }
    }

    fn update(&mut self, _delta: f32) {}
}
