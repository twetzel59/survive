use sfml::graphics::RenderTarget;

pub trait MousePosHandler {
    fn mouse_pos(&mut self, target: &RenderTarget, mx: i32, my: i32);
}

/*
pub trait MouseClickHandler {
    fn mouse_click(&mut self, target: &RenderTarget, mx: i32, my: i32);
}
*/
