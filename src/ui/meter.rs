use sfml::graphics::*;
use resizehandler::ResizeHandler;
use resources::Resources;
use super::Element;

pub struct Meter<'s> {
    icon: Sprite<'s>,
    outer: RectangleShape<'s>,
    inner: RectangleShape<'s>,
}

impl<'s> Meter<'s> {
    pub fn new(res: &'s Resources) -> Meter<'s> {
        Meter {
            icon: Sprite::with_texture(&res.ui.hydration),
            outer: RectangleShape::new(),
            inner: RectangleShape::new(),
        }
    }
}

impl<'s> ResizeHandler for Meter<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.icon.set_position2f(0., 0.);
    }
}

impl<'s, T: RenderTarget> Element<T> for Meter<'s> {
    fn draw(&self, target: &mut T) {
        target.draw(&self.icon);
        target.draw(&self.outer);
        target.draw(&self.inner);
    }
}
