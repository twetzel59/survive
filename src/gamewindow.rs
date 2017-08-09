use sfml::graphics::*;
use sfml::window::*;
use sfml::system::{Vector2i, Vector2f};

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
    
    pub fn scroll(&mut self, dir: Vector2f) {
        let view = self.rwin.view().to_owned();
    }
}
