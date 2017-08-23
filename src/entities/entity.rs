use sfml::graphics::{RenderTarget, Sprite};

pub trait Entity<'s> {
    fn draw(&self, target: &mut RenderTarget);
    fn sprite(&self) -> &Sprite<'s>;
    //fn sprite_mut(&mut self) -> &mut Sprite<'s>;
    fn on_click(&mut self);
}
