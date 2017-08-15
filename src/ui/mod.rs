pub mod ui_manager;

mod meter;

use sfml::graphics::RenderTarget;
use sfml::system::{Vector2f, Vector2u};
use resizehandler::ResizeHandler;

trait Element<T: RenderTarget>: ResizeHandler {
    fn draw(&self, target: &mut T);
    fn set_position_relative(&mut self, pos: &Vector2f, win_size: &Vector2u);
}
