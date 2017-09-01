use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};
use resize_handler::ResizeHandler;
use super::element::*;

const OFFSET: Vector2f = Vector2f { x: 32., y: 32. };

pub struct Counter<'s> {
    icon: Sprite<'s>,
    text: Text<'s>,
    rel_pos: Vector2f,
}

impl<'s> Counter<'s> {
    pub fn new(tex: &'s TextureRef, font: &'s Font, rel_pos: &Vector2f) -> Counter<'s> {
        Counter {
            icon: Sprite::with_texture(tex),
            text: Text::new_init("  0", font, 32),
            rel_pos: *rel_pos,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        let string = if value <= 9 {
            format!("  {}", value)
        } else if value <= 99 {
            format!(" {}", value)
        } else {
            format!("{}", value)
        };

        self.text.set_string(&string);
    }

    fn recalculate(&mut self, win_width: u32, win_height: u32) {
        self.icon.set_position2f(win_width as f32 * self.rel_pos.x,
                                 win_height as f32 * self.rel_pos.y);

        let bounds = self.icon.global_bounds();
        self.text.set_position2f(bounds.left, bounds.top);
        self.text.move_(&OFFSET);
    }
}

impl<'s> ResizeHandler for Counter<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.recalculate(width, height);
    }
}

impl<'s> UiDrawable for Counter<'s> {
    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.icon);
        target.draw(&self.text);
    }
}

impl<'s> Element for Counter<'s> {
    fn set_position_relative(&mut self, pos: &Vector2f, win_size: &Vector2u) {
        self.rel_pos = *pos;
        self.recalculate(win_size.x, win_size.y);
    }
}
