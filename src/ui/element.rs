use sfml::graphics::RenderTarget;
use sfml::system::{Vector2f, Vector2u};
use resize_handler::ResizeHandler;

pub trait Element: ResizeHandler {
    fn draw(&self, target: &mut RenderTarget);
    fn set_position_relative(&mut self, pos: &Vector2f, win_size: &Vector2u);
}
