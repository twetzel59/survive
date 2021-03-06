use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};
use resize_handler::ResizeHandler;
use super::element::*;

const PADDING: f32 = 10.;
const BORDER: f32 = 1.;
const WIDTH: f32 = 100.;

pub struct Meter<'s> {
    icon: Sprite<'s>,
    outer: RectangleShape<'s>,
    inner: RectangleShape<'s>,
    rel_pos: Vector2f,
}

impl<'s> Meter<'s> {
    pub fn new(tex: &'s TextureRef, style: &MeterStyle, rel_pos: &Vector2f) -> Meter<'s> {
        let size = tex.size();

        let mut outer = RectangleShape::new();
        outer.set_size2f(WIDTH, size.y as f32);
        outer.set_fill_color(&style.outline);
        //outer.set_fill_color(&Color::rgb(30, 144, 255));

        let mut inner = RectangleShape::new();
        inner.set_size2f(WIDTH - BORDER * 2., size.y as f32 - BORDER * 2.);
        inner.set_fill_color(&style.fill);
        //inner.set_fill_color(&Color::rgb(18, 86, 153));

        Meter {
            icon: Sprite::with_texture(tex),
            outer,
            inner,
            rel_pos: *rel_pos,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        if value < 0. || value > 1. {
            panic!("Invalid meter value.");
        }

        let height = self.inner.size().y;
        self.inner.set_size2f((WIDTH * value - BORDER * 2.).max(0.), height);
    }

    fn recalculate(&mut self, win_width: u32, win_height: u32) {
        self.icon.set_position2f(win_width as f32 * self.rel_pos.x,
                                 win_height as f32 * self.rel_pos.y);

        let bounds = self.icon.global_bounds();
        self.outer.set_position2f(bounds.left + bounds.width + PADDING,
                                  bounds.top);
        let pos = self.outer.position();
        self.inner.set_position2f(pos.x + BORDER, pos.y + BORDER);
    }
}

impl<'s> ResizeHandler for Meter<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.recalculate(width, height);
    }
}

impl<'s> UiDrawable for Meter<'s> {
    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.icon);
        target.draw(&self.outer);
        target.draw(&self.inner);
    }
}

impl<'s> Element for Meter<'s> {
    fn set_position_relative(&mut self, pos: &Vector2f, win_size: &Vector2u) {
        self.rel_pos = *pos;
        self.recalculate(win_size.x, win_size.y);
    }
}

pub struct MeterStyle {
    pub fill: Color,
    pub outline: Color,
}
