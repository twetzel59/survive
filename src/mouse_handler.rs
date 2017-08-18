use sfml::graphics::RenderTarget;

pub trait MousePosHandler {
    fn mouse_pos(&mut self, target: &RenderTarget, mx: i32, my: i32);
}
