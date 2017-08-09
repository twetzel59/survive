use sfml::graphics::RenderTarget;

pub trait MouseMoveHandler {
    fn on_mouse_move(&mut self, target: &RenderTarget, mx: i32, my: i32);
}
