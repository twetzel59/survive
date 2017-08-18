use sfml::graphics::{Drawable, Sprite};

pub trait Entity<'s>: Drawable {
    fn sprite(&self) -> &Sprite<'s>;
    fn sprite_mut(&mut self) -> &mut Sprite<'s>;
}
