use sfml::graphics::{Color, RenderTarget, Text, Transformable};
use resize_handler::ResizeHandler;
use super::element::UiDrawable;
use resources::Resources;

pub struct DeathScreen<'s> {
    text: Text<'s>
}

impl<'s> DeathScreen<'s> {
    pub fn new(res: &'s Resources) -> DeathScreen<'s> {
        let mut text = Text::new_init("#YouDied", &res.fnt.normal, 60);
        text.set_fill_color(&Color::rgba(255, 255, 255, 0));

        DeathScreen {
            text,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let mut c = self.text.fill_color();
        let a = c.a as f32 + 255. * delta;
        if a <= 255. {
            c.a = a as u8;
            self.text.set_fill_color(&c);
        }
    }
}

impl<'s> ResizeHandler for DeathScreen<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        let bounds = self.text.global_bounds();

        self.text.set_position2f(width as f32 / 2. - bounds.width / 2.,
                                 height as f32 / 2. - bounds.height);
                                 // No division for the sake of
                                 // visual centering.
    }
}

impl<'s> UiDrawable for DeathScreen<'s> {
    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.text);
    }
}
