use sfml::graphics::*;
use sfml::window::*;
use sfml::system::{Vector2i, Vector2f, Vector2u};
use resizehandler::ResizeHandler;

const TITLE: &'static str = "#survive";
const DEFAULT_SIZE: (u32, u32) = (800, 600);

/*
#[derive(Clone, Copy)]
pub enum ScrollDir {
    Up,
    Left,
    Down,
    Right,
}
*/

pub struct GameWindow {
    pub rwin: RenderWindow,
}

impl GameWindow {
    pub fn new() -> GameWindow {
        let mut rwin = RenderWindow::new(VideoMode::new(
                                            DEFAULT_SIZE.0, DEFAULT_SIZE.1, 32),
                                    TITLE,
                                    style::DEFAULT,
                                    &Default::default()).unwrap();
        rwin.set_vertical_sync_enabled(true);
        
        let mut view = rwin.view().to_owned();
        //view.set_center2f(DEFAULT_SIZE.0 as f32 / 2., DEFAULT_SIZE.1 as f32 / 2.);
        view.set_center2f(0., 0.);
        rwin.set_view(&view);
        
        rwin.set_mouse_position(&Vector2i::new(
                                DEFAULT_SIZE.0 as i32 / 2, (DEFAULT_SIZE.1 as i32 / 2) - 50));
        
        GameWindow {
            rwin,
        }
    }
    
    pub fn scroll(&mut self, dir: &Vector2f) {
        let mut view = self.rwin.view().to_owned();
        view.move_(dir);
        self.rwin.set_view(&view);
    }
    
    pub fn scroll_bounds(&self, bound: f32) -> ScrollBounds {
        let Vector2u { x: sx, y: sy } = self.rwin.size();
        let size = Vector2f::new(sx as f32, sy as f32);
        
        ScrollBounds {
            left: FloatRect::new(0., 0., bound, size.y),
            top: FloatRect::new(0., 0., size.x, bound),
            bottom: FloatRect::new(0., size.y - bound, size.x, bound),
            right: FloatRect::new(size.x - bound, 0., bound, size.y),
        }
    }
}

impl ResizeHandler for GameWindow {
    fn on_resize(&mut self, width: u32, height: u32) {
        let mut view = self.rwin.view().to_owned();
        view.set_size2f(width as f32, height as f32);
        self.rwin.set_view(&view);
    }
}

pub struct ScrollBounds {
    pub left: FloatRect,
    pub top: FloatRect,
    pub bottom: FloatRect,
    pub right: FloatRect,
}
