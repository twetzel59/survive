use sfml::graphics::{RenderTarget};

pub trait Entity<'s> {
    fn draw(&self, target: &mut RenderTarget);
    //fn sprite(&self) -> &Sprite<'s>;
    //fn sprite_mut(&mut self) -> &mut Sprite<'s>;
}
