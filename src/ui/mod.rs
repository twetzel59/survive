pub mod ui_manager;

mod meter;

use sfml::graphics::RenderTarget;
use resizehandler::ResizeHandler;

trait Element<T: RenderTarget>: ResizeHandler {
    fn draw(&self, target: &mut T);
}
