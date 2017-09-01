use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2u};
use resize_handler::ResizeHandler;
use resources::Resources;
use super::element::*;

const INNER_RADIUS: f32 = 30.;
const OUTER_RADIUS: f32 = 38.;
const POINT_COUNT: u32 = 180;
const OUTLINE_THICKNESS: f32 = 4.;
const OUTLINE_COLOR: Color = Color { r: 200, g: 200, b:  200, a: 255 };
const SHADOW_COLOR:  Color = Color { r:   0, g:   0, b:   25, a:  25 };

pub struct Sundial<'s> {
    inner: CircleShape<'s>,
    outer: CircleShape<'s>,
    rel_pos: Vector2f,
}

impl<'s> Sundial<'s> {
    pub fn new(res: &'s Resources, rel_pos: &Vector2f) -> Sundial<'s> {
        let mut d = Sundial {
            inner: CircleShape::new_init(INNER_RADIUS, POINT_COUNT),
            outer: CircleShape::new_init(OUTER_RADIUS, POINT_COUNT),
            rel_pos: *rel_pos,
        };

        d.inner.set_texture(&res.ui.day_night, true);
        d.inner.set_outline_thickness(OUTLINE_THICKNESS);
        d.inner.set_outline_color(&OUTLINE_COLOR);
        d.inner.set_origin2f(INNER_RADIUS, INNER_RADIUS);

        d.outer.set_fill_color(&SHADOW_COLOR);
        d.outer.set_origin2f(OUTER_RADIUS * 2., 0.);

        d
    }

    pub fn set_daytime(&mut self, time: f32) {
        self.inner.set_rotation(time * 360.);
    }

    fn recalculate(&mut self, win_width: u32, win_height: u32) {
        let win_width = win_width as f32;
        let win_height = win_height as f32;

        let pos = Vector2f::new(win_width - win_width * self.rel_pos.x,
                                win_height * self.rel_pos.y);

        self.inner.set_position2f(pos.x - OUTLINE_THICKNESS * 2. - INNER_RADIUS,
                                  pos.y + OUTLINE_THICKNESS * 2. + INNER_RADIUS);
        self.outer.set_position(&pos);
    }
}

impl<'s> ResizeHandler for Sundial<'s> {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.recalculate(width, height);
    }
}

impl<'s> UiDrawable for Sundial<'s> {
    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.outer);
        target.draw(&self.inner);
    }
}

impl<'s> Element for Sundial<'s> {
    fn set_position_relative(&mut self, pos: &Vector2f, win_size: &Vector2u) {
        self.rel_pos = *pos;
        self.recalculate(win_size.x, win_size.y);
    }
}
