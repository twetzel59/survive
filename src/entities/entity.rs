use sfml::graphics::{RenderTarget, Sprite};
use registry::item::{Item, Stack};

pub trait Entity<'s> {
    fn draw(&self, target: &mut RenderTarget);
    fn sprite(&self) -> &Sprite<'s>;
    //fn sprite_mut(&mut self) -> &mut Sprite<'s>;

    fn care_about_click(&self) -> bool {
        false
    }

    fn on_click(&mut self) -> Stack {
        Stack { item: Item::Wood, count: 0 }
    }
}
